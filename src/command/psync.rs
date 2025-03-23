use crate::command::Command;
use crate::resp::RespData;

impl Command {
    pub fn psync(&self) -> RespData {
        RespData::SimpleString("OK".to_string())
    }
}