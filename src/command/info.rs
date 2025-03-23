use crate::command::Command;
use crate::resp::RespData;
use crate::storage::Db;

impl Command {
    pub fn info(&self, storage: &Db) -> RespData {
        match storage.config_get(&"replicaof".to_string()) {
            Some(_v) => RespData::BulkString("role:slave".to_string()),
            None => RespData::BulkString("role:master".to_string()),
        }
    }
}