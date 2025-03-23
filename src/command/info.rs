use crate::command::Command;
use crate::{DEFAULT_MASTER_OFFSET, DEFAULT_MASTER_REPLID};
use crate::resp::RespData;
use crate::storage::Db;

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