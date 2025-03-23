use crate::command::Command;
use crate::{DEFAULT_MASTER_OFFSET, DEFAULT_MASTER_REPLID};
use crate::resp::RespData;

impl Command {
    pub fn psync(&self) -> RespData {
        RespData::SimpleString(format!(
            "FULLRESYNC {} {}", DEFAULT_MASTER_REPLID, DEFAULT_MASTER_OFFSET
        ))
    }
}