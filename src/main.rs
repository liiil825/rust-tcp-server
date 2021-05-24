use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // create a TcpListener
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                print(s);
            }
            Err(e) => { 
                panic!("crash incoming error: {}", e);
            }
        }
    }
}

fn print(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    
    println!("get message: {}", String::from_utf8_lossy(&buffer[..]));
}
