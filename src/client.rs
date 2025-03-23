use crate::resp::{RespData, RespHandler};
use std::net::TcpStream;

pub struct ReplicaClient {
    handler: RespHandler,
}

impl ReplicaClient {
    pub fn connect(address: String) -> Self {
        let sock = TcpStream::connect(address).unwrap();
        let handler = RespHandler::new(sock);
        ReplicaClient { handler }
    }

    pub fn ping(&mut self) -> RespData {
        let command = RespData::Array(vec![RespData::BulkString("PING".to_string())]);
        self.call(command)
    }

    pub fn replconf(&mut self, key: String, value: String) -> RespData {
        let mut items = vec![];
        items.push(RespData::BulkString("REPLCONF".to_string()));
        items.push(RespData::BulkString(key));
        items.push(RespData::BulkString(value));
        let command = RespData::Array(items);
        self.call(command)
    }

    pub fn psync(&mut self) -> RespData {
        let mut items = vec![];
        items.push(RespData::BulkString("PSYNC".to_string()));
        items.push(RespData::BulkString("?".to_string()));
        items.push(RespData::BulkString("-1".to_string()));
        let command = RespData::Array(items);
        self.call(command)
    }

    fn call(&mut self, cmd: RespData) -> RespData {
        self.handler.write_value(cmd).unwrap();
        let resp = self.handler.read_value().unwrap();
        if let Some(v) = resp {
            v
        } else {
            RespData::Null
        }
    }
}