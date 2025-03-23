mod get;
mod set;
mod ping;
mod echo;
mod config;
mod keys;

use crate::resp::RespData;
use crate::storage::Db;

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
            "ping" => self.ping(),
            "echo" => self.echo(),
            "set" => self.set(storage),
            "get" => self.get(storage),
            "config" => self.config(storage),
            "keys" => self.keys(storage),
            c => RespData::Error(format!("ERR Cannot handle command {c}")),
        }
    }
}

fn extract_command(value: RespData) -> (String, Vec<RespData>) {
    match value {
        RespData::Array(a) => (
            a.first().unwrap().unpack_str(),
            a.into_iter().skip(1).collect(),
        ),
        _ => panic!("Unexpected command format"),
    }
}

