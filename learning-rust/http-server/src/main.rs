use std::net::TcpListener;
use threadpool::ThreadPool;

mod http_handling;
mod threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let mut pool = ThreadPool::new(20);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            http_handling::handle_connection(stream);
        });
    }
}
