mod resp;
mod command;
mod db;

use std::net::{TcpListener, TcpStream};
use crate::command::Command;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // spawns a new thread for each incoming connection
                // 'move' lets the closure take ownership of 'stream'
                let _ = std::thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut handler = resp::RespHandler::new(stream);
    let mut storage= db::Db::new();

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



