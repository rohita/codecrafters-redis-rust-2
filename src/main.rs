mod resp;

use std::net::{TcpListener, TcpStream};
use anyhow::{Result};
use crate::resp::RespType;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut handler = resp::RespHandler::new(stream);
    loop {
        let value = handler.read_value().unwrap();
        println!("Got value {:?}", value);
        let response = if let Some(v) = value {
            let (command, args) = extract_command(v).unwrap();
            match command.to_lowercase().as_str() {
                "ping" => RespType::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).unwrap();
    }
}

fn extract_command(value: RespType) -> Result<(String, Vec<RespType>)> {
    match value {
        RespType::Array(a) => Ok((
            unpack_bulk_str(a.first().unwrap().clone())?,
            a.into_iter().skip(1).collect(),
        )),
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

fn unpack_bulk_str(value: RespType) -> Result<String> {
    match value {
        RespType::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Expected command to be a bulk string")),
    }
}
