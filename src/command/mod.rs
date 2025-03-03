mod get;
mod set;

use crate::resp::RespData;
use crate::db::Db;

pub struct Command {
    name: String,
    args: Vec<RespData>,
}

impl Command {
    pub fn new(input: RespData) -> Command {
        let (name, args) = extract_command(input);
        Command { name, args }
    }

    pub fn handle(&mut self, storage: &mut Db) -> RespData {
        match self.name.to_lowercase().as_str() {
            "ping" => RespData::SimpleString("PONG".to_string()),
            "echo" => self.args.first().unwrap().clone(),
            "set" => self.set(storage),
            "get" => self.get(storage),
            c => RespData::Error(format!("ERR Cannot handle command {c}")),
        }
    }
}

fn extract_command(value: RespData) -> (String, Vec<RespData>) {
    match value {
        RespData::Array(a) => (
            a.first().unwrap().clone().unpack_str(),
            a.into_iter().skip(1).collect(),
        ),
        _ => panic!("Unexpected command format"),
    }
}

