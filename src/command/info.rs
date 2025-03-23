use crate::command::Command;
use crate::resp::RespData;

impl Command {
    pub fn info(&self) -> RespData {
        RespData::BulkString("role:master".to_string())
    }
}