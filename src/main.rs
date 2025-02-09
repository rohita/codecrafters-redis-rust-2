use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 512];
    loop {
        let read_count = stream.read(&mut buf).unwrap();
        if read_count == 0 {
            break;
        }
        // "b" make it a byte string
        // "unwrap" assert that no errors occur during the write operation.
        // If an error does occur, it will cause the program to panic and exit.
        stream.write_all(b"+PONG\r\n").unwrap();
    }
}
