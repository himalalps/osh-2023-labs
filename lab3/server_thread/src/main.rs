use server::*;
use std::{net::TcpListener, thread};

const BIND_IP: &str = "0.0.0.0:8000";

fn main() {
    let listener =
        TcpListener::bind(BIND_IP).expect(format!("Failed to bind to {}.", BIND_IP).as_str());

    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed.");

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
