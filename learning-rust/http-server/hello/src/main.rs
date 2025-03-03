use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "Connection established.\nI should interpret the request and send a response to {}\n",
            stream.peer_addr().unwrap().to_string()
        );
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader_lines_iterator = BufReader::new(&stream).lines();

    let request_line = buf_reader_lines_iterator.next().unwrap().unwrap();
    println!("{}\r\n", request_line);

    if request_line == "GET / HTTP/1.1" {
        stream.write_file_response(
            "HTTP/1.1 200 OK",
            "resources/hello.html",
        )
    } else {
        stream.write_file_response(
            "HTTP/1.1 404 NOT FOUND",
            "resources/404.html",
        );
    }
}

trait ResponseFile {
    fn write_file_response(
        &mut self,
        status_line: &str,
        file_path: &str,
    );
}

impl ResponseFile for TcpStream {
    fn write_file_response(
        &mut self,
        status_line: &str,
        file_path: &str,
    ) {
        let file = fs::read(file_path).unwrap();
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            file.len()
        );

        self.write_all(response.as_bytes()).unwrap();
        self.write_all(&file[..]).unwrap();
    }
}