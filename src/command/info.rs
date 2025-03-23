use crate::command::Command;
use crate::resp::RespData;
use crate::storage::Db;

const DEFAULT_MASTER_REPLID: &str = "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb";
const DEFAULT_MASTER_OFFSET: &str = "0";

impl Command {
    pub fn info(&self, storage: &Db) -> RespData {
        let role = match storage.config_get(&"replicaof".to_string()) {
            Some(_v) => "slave",
            None => "master",
        };

        RespData::BulkString(
            format!("role:{}\r\nmaster_replid:{}\r\nmaster_repl_offset:{}", role, DEFAULT_MASTER_REPLID, DEFAULT_MASTER_OFFSET)
        )
    }
}