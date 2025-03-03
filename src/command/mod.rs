use std::collections::HashMap;
use crate::resp::RespType;

pub fn handle(value: RespType, storage: &mut HashMap<String, String>) -> RespType {
    let (command, args) = extract_command(value).unwrap();

    match command.to_lowercase().as_str() {
        "ping" => RespType::SimpleString("PONG".to_string()),
        "echo" => args.first().unwrap().clone(),
        "set" => set(storage, args[0].clone().unpack_str(), args[1].clone().unpack_str()),
        "get" => get(&storage, args[0].clone().unpack_str()),
        _ => RespType::Error(format!("ERR Cannot handle command {command}")),
    }
}


fn extract_command(value: RespType) -> anyhow::Result<(String, Vec<RespType>)> {
    match value {
        RespType::Array(a) => Ok((
            a.first().unwrap().clone().unpack_str(),
            a.into_iter().skip(1).collect(),
        )),
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}

fn set(
    storage: &mut HashMap<String, String>,
    key: String,
    value: String,
) -> RespType {
    storage.insert(key, value);
    RespType::SimpleString("OK".to_string())
}

fn get(storage: &HashMap<String, String>, key: String) -> RespType {
    match storage.get(&key) {
        Some(v) => RespType::BulkString(v.to_string()),
        None => RespType::Null,
    }
}