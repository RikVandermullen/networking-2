use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    println!("Connected to server! {}", socket.local_addr().unwrap());

    socket.send_to(b"Ping", "127.0.0.1:12345").expect("couldn't send data");

    let mut buf = [0; 2048];
    socket.recv_from(&mut buf).expect("Didn't receive data");

    println!("Server Sent: {}", String::from_utf8_lossy(&buf));
}
