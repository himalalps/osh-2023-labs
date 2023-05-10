pub mod pool;

use pool::ThreadPool;
use server::*;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{net::TcpListener, process::exit};

const BIND_IP: &str = "0.0.0.0:8000";

fn main() {
    let listener =
        TcpListener::bind(BIND_IP).expect(format!("Failed to bind to {}.", BIND_IP).as_str());
    let pool = ThreadPool::new(32);

    pool.execute(|| {
        let mut signals = Signals::new(&[SIGINT]).unwrap();
        for _signal in signals.forever() {
            // drop(pool);
        }
    });

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
