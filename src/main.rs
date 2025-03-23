mod resp;
mod command;
mod storage;

use std::collections::HashMap;
use std::env;
use std::net::{TcpListener, TcpStream};
use crate::command::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = HashMap::<String, String>::new();
    for index in (1..args.len()).step_by(2) {
        let config_key = args[index].get(2..).unwrap().to_string();
        config.insert(config_key, args[index + 1].to_string());
    }
    println!("Config: {:?}", config);
    let storage= storage::Db::from_config(config);

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

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



