use std::fmt::format;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(err) => eprintln!("Connection failed, {err}"),
        }

        println!("Connection established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "assets/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "assets/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}
