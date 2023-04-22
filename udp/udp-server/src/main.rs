use std::net::UdpSocket;

fn handler(socket: &UdpSocket) -> std::io::Result<()> {
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    println!("New connection {}", src);

    let buf = &mut buf[..amt];

    println!("Client Sent: {:?}", String::from_utf8_lossy(&buf));

    socket.send_to(b"Pong", &src)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    {
        println!("Starting listener...");
        let socket = UdpSocket::bind("127.0.0.1:12345")?;
        while true {
            handler(&socket)?;
        }
    }
    Ok(())
}
