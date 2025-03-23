mod resp;
mod command;
mod storage;
mod client;

use crate::command::Command;
use std::collections::HashMap;
use std::env;
use std::net::{TcpListener, TcpStream};

const DEFAULT_MASTER_REPLID: &str = "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb";
const DEFAULT_MASTER_OFFSET: &str = "0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = HashMap::<String, String>::new();
    for index in (1..args.len()).step_by(2) {
        let config_key = args[index].get(2..).unwrap().to_string();
        config.insert(config_key, args[index + 1].to_string());
    }
    println!("Config: {:?}", config);

    let port: String = config.get("port").unwrap_or(&"6379".to_string()).to_string();
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    if let Some(addr) = config.get("replicaof") {
        let addr = addr.replace(' ', ":");
        let mut master = client::ReplicaClient::connect(addr);
        master.ping();
        master.replconf("listening-port".to_string(), port);
        master.replconf("capa".to_string(), "psync2".to_string());
        master.psync();
    }

    let storage= storage::Db::from_config(config);
    for stream in listener.incoming() {
        let storage = storage.clone();  // Can we avoid cloning here??

        match stream {
            Ok(stream) => {
                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream, storage));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, mut storage: storage::Db) {
    let mut handler = resp::RespHandler::new(stream);

    loop {
        let value = handler.read_value().unwrap();
        println!("Got value {:?}", value);
        let response = if let Some(v) = value {
            let mut command = Command::new(v);
            command.handle(&mut storage)
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).unwrap();
    }
}



