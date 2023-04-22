use std::{net::{TcpListener, TcpStream}};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }
        stream.write(b"Pong").expect("Failed to write to stream");
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }
}

pub fn main() -> std::io::Result<()> {
    println!("Starting listener...");
    let listener = TcpListener::bind("127.0.0.1:12345").expect("Failed to bind to address");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        handle_client(stream);
    }
    Ok(())
}