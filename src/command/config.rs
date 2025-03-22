use crate::command::Command;
use crate::db::Db;
use crate::resp::RespData;

impl Command {
    pub fn config(&self, storage: &Db) -> RespData {
        let subcommand = self.args[0].unpack_str();
        let first = self.args[1].clone();

        match subcommand.as_str() {
            "GET" => {
                match storage.config_get(&first.unpack_str()) {
                    Some(v) => {
                        let mut items = vec![];
                        items.push(first);
                        items.push(RespData::BulkString(v.to_string()));
                        RespData::Array(items)
                    },
                    None => RespData::Null,
                }
            },
            _ => RespData::Null
        }
    }
}