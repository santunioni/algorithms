use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!(
            "Connection established. I should interpret the request and send a response to {:?}",
            _stream.peer_addr().unwrap()
        );
    }
}
