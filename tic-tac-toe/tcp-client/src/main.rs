use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;

pub fn main() -> std::io::Result<()> {
    println!("Connecting to server...");

    let stream = TcpStream::connect("127.0.0.1:12345").expect("Failed to connect to server");

    println!("Connected to server! {}", stream.peer_addr().unwrap());
    
    let mut line = String::new();    
    loop {
        if line.trim() == "exit" {
            println!("Shutting down connection...");
            break;
        } else {
            println!("Play turn (x-y-player): ");
            std::io::stdin().read_line(&mut line).unwrap();
            send_data(&stream, line.as_bytes());
            line = String::new();
        }

    }
    Ok(())
}

pub fn send_data(mut stream: &TcpStream, data: &[u8]) {
    stream.write(data).expect("Failed to write to stream");
    
    receive_data(stream);
}

pub fn receive_data(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    let n: usize = stream.read(&mut buffer).expect("Failed to read from stream");

    println!("Received:");
    println!("{}", String::from_utf8_lossy(&buffer[..n]));

}
