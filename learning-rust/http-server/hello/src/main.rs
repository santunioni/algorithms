use std::net::TcpListener;
use threadpool::ThreadPool;

mod threadpool;
mod http_handling;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "Connection established.\nI should interpret the request and send a response to {}\n",
            stream.peer_addr().unwrap().to_string()
        );
        pool.execute(|| {
            http_handling::handle_connection(stream);
        });
    }
}


