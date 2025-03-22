use anyhow::Result;
use bytes::{BufMut, BytesMut};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Clone, Debug)]
pub enum RespData {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<RespData>),
    Null,
}

impl RespData {
    pub fn encode(self) -> String {
        match self {
            RespData::SimpleString(s) => format!("+{}\r\n", s),
            RespData::Error(s) => format!("-{}\r\n", s),
            RespData::Integer(i) => format!(":{}\r\n", i),
            RespData::BulkString(s) => format!("${}\r\n{}\r\n", s.chars().count(), s),
            RespData::Array(v) => format!("*{}\r\n{}",
                                          v.len(), v.into_iter().map(|s| s.encode()).collect::<Vec<_>>().join("")),
            RespData::Null => "$-1\r\n".to_string(),
        }
    }

    pub fn unpack_str(&self) -> String {
        match self {
            RespData::SimpleString(s) => s.to_string(),
            RespData::BulkString(s) => s.to_string(),
            _ => panic!("Expected command to be a simple or bulk string")
        }
    }
}

pub struct RespHandler {
    stream: TcpStream,
    buffer: BytesMut,
}

impl RespHandler {
    pub fn new(stream: TcpStream) -> Self {
        RespHandler {
            stream,
            buffer: BytesMut::with_capacity(512),
        }
    }

    pub fn read_value(&mut self) -> Result<Option<RespData>> {
        let mut buf = [0; 512];
        let bytes_read = self.stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(None);
        }
        self.buffer.put_slice(&buf[..bytes_read]);
        let (v, _) = parse_message(self.buffer.split())?;
        Ok(Some(v))
    }

    pub fn write_value(&mut self, value: RespData) -> Result<()> {
        self.stream.write_all(value.encode().as_bytes())?;
        Ok(())
    }
}

fn parse_message(buffer: BytesMut) -> Result<(RespData, usize)> {
    match buffer[0] as char {
        '+' => parse_simple_string(buffer),
        '*' => parse_array(buffer),
        '$' => parse_bulk_string(buffer),
        _ => Err(anyhow::anyhow!("Not a known value type {:?}", buffer)),
    }
}

fn parse_simple_string(buffer: BytesMut) -> Result<(RespData, usize)> {
    if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let string = String::from_utf8(line.to_vec()).unwrap();
        return Ok((RespData::SimpleString(string), len + 1))
    }
    return Err(anyhow::anyhow!("Invalid string {:?}", buffer));
}

fn parse_array(buffer: BytesMut) -> Result<(RespData, usize)> {
    let (array_length, mut bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let array_length = parse_int(line)?;
        (array_length, len + 1)
    } else {
        return Err(anyhow::anyhow!("Invalid array format {:?}", buffer));
    };
    let mut items = vec![];
    for _ in 0..array_length {
        let (array_item, len) = parse_message(BytesMut::from(&buffer[bytes_consumed..]))?;
        items.push(array_item);
        bytes_consumed += len;
    }
    return Ok((RespData::Array(items), bytes_consumed))
}

fn parse_bulk_string(buffer: BytesMut) -> Result<(RespData, usize)> {
    let (bulk_str_len, bytes_consumed) = if let Some((line, len)) = read_until_crlf(&buffer[1..]) {
        let bulk_str_len = parse_int(line)?;
        (bulk_str_len, len + 1)
    } else {
        return Err(anyhow::anyhow!("Invalid array format {:?}", buffer));
    };
    let end_of_bulk_str = bytes_consumed + bulk_str_len as usize;
    let total_parsed = end_of_bulk_str + 2;
    Ok((RespData::BulkString(String::from_utf8(buffer[bytes_consumed..end_of_bulk_str].to_vec())?), total_parsed))
}
fn read_until_crlf(buffer: &[u8]) -> Option<(&[u8], usize)> {
    for i in 1..buffer.len() {
        if buffer[i - 1] == b'\r' && buffer[i] == b'\n' {
            return Some((&buffer[0..(i - 1)], i + 1));
        }
    }
    return None;
}
fn parse_int(buffer: &[u8]) -> Result<i64> {
    Ok(String::from_utf8(buffer.to_vec())?.parse::<i64>()?)
}