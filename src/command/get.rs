use crate::command::Command;
use crate::db::Db;
use crate::resp::RespData;

impl Command {
    pub fn get(&self, storage: &Db) -> RespData {
        let key = self.args[0].clone().unpack_str();
        match storage.get(key) {
            Some(v) => RespData::BulkString(v.to_string()),
            None => RespData::Null,
        }
    }
}