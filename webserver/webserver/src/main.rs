use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};



fn main() {
    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();

    let html_files = get_html_files();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &html_files);
    }
}

fn handle_connection(mut stream: TcpStream, html_files: &Vec<String>) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let route = request_line.split_whitespace().nth(1).unwrap();
    for html_file in html_files {
        if html_file.contains(route) {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string(html_file).unwrap();
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            stream.write_all(response.as_bytes()).unwrap();
            return;
        } else {
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let response = format!(
                "{status_line}\r\n"
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn get_html_files() -> Vec<String> {
    let mut html_files = Vec::new();
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let found_path = &path.unwrap().path().display().to_string();
        if found_path.ends_with(".html") {
            html_files.push(found_path.to_string());
        }
    }

    return html_files;
}