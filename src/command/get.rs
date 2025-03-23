use crate::command::Command;
use crate::storage::Db;
use crate::resp::RespData;

impl Command {
    pub fn get(&self, storage: &Db) -> RespData {
        let key = self.args[0].unpack_str();
        match storage.get(&key) {
            Some(v) => RespData::BulkString(v.to_string()),
            None => RespData::Null,
        }
    }
}