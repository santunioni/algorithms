use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use std::{fs, thread};
use std::time::Duration;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader_lines_iterator = BufReader::new(&stream).lines();

    let request_line = buf_reader_lines_iterator.next().unwrap().unwrap();
    println!("{}\r\n", request_line);

    match &request_line[..] {
        "GET / HTTP/1.1" => stream.write_file_response(
            "HTTP/1.1 200 OK",
            "resources/hello.html",
        ),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            stream.write_file_response(
                "HTTP/1.1 200 OK",
                "resources/hello.html",
            )
        }
        _ => stream.write_file_response(
            "HTTP/1.1 404 NOT FOUND",
            "resources/404.html",
        ),
    };
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