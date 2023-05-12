use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let route = request_line.split_whitespace().nth(1).unwrap();
    
    send_response(&mut stream, route);    
}

fn send_response(stream: &mut TcpStream, route: &str) {
    let status_line = "HTTP/1.1 200 OK";

    let mut file_path = format!(".{}", route);
    if !route.contains(".") {
        file_path = format!("{}.html", file_path);
    }

    let contents = fs::read_to_string(file_path);
    match contents {
        Ok(contents) => {
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
        },
        Err(_) => {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let content = "404 NOT FOUND";
            let length = content.len();
            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
            );
            stream.write_all(response.as_bytes()).unwrap();
        },
    };    
}

