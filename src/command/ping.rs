use crate::command::Command;
use crate::resp::RespData;

impl Command {
    pub fn ping(&self) -> RespData {
        RespData::SimpleString("PONG".to_string())
    }
}