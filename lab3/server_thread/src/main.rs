pub mod pool;

use pool::ThreadPool;
use server::*;
use std::net::TcpListener;

const BIND_IP: &str = "0.0.0.0:8000";

fn main() {
    let listener =
        TcpListener::bind(BIND_IP).expect(format!("Failed to bind to {}.", BIND_IP).as_str());
    let pool = ThreadPool::new(32);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(tcpstream) => tcpstream,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
