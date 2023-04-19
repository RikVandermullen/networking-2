use std::io::prelude::*;
use std::net::TcpStream;

pub fn main() -> std::io::Result<()> {
    println!("Connecting to server...");

    let mut stream = TcpStream::connect("127.0.0.1:12345").expect("Failed to connect to server");

    println!("Connected to server! {}", stream.peer_addr().unwrap());
    
    stream.write(b"Hello World!")?;

    let mut buffer = [0; 1024];
    let n: usize = stream.read(&mut buffer)?;

    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
