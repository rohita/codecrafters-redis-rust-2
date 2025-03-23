use crate::command::Command;
use crate::{DEFAULT_MASTER_OFFSET, DEFAULT_MASTER_REPLID};
use crate::resp::RespData;
use crate::storage::Db;

impl Command {
    pub fn psync(&self, storage: &Db) -> RespData {
        let mut items = vec![];

        let fullsresync = RespData::SimpleString(format!("FULLRESYNC {} {}", DEFAULT_MASTER_REPLID, DEFAULT_MASTER_OFFSET));
        items.push(fullsresync);

        let file = RespData::File(storage.get_rdb_file());
        items.push(file);

        RespData::Multipart(items)
    }
}