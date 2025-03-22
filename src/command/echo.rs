use crate::command::Command;
use crate::resp::RespData;

impl Command {
    pub fn echo(&self)  -> RespData {
        self.args.first().unwrap().clone()
    }
}