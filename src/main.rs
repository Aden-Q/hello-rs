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
    let _http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("assets/index.html").unwrap();
    let length = contents.len();

    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n\n{contents}");

    stream.write_all(resp.as_bytes()).unwrap();
}
