use crate::command::Command;
use crate::storage::Db;
use crate::resp::RespData;

impl Command {
    pub fn keys(&self, storage: &Db) -> RespData {
        let keys = storage
            .keys()
            .iter()
            .map(|s| RespData::BulkString(s.to_string()))
            .collect::<Vec<_>>();
        RespData::Array(keys)
    }
}