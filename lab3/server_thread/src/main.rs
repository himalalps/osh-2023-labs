#[cfg(test)]
mod test;

use server_thread::*;
use std::net::TcpListener;

const BIND_IP: &str = "0.0.0.0:8000";

fn main() {
    let listener = TcpListener::bind(BIND_IP).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
