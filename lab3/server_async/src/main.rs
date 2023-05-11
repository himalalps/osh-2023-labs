use server::*;
use tokio::net::TcpListener;

const BIND_IP: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(BIND_IP)
        .await
        .expect(format!("Failed to bind to {}.", BIND_IP).as_str());

    loop {
        let stream = match listener.accept().await {
            Ok((tcpstream, _)) => tcpstream,
            Err(e) => {
                eprintln!("Err: {}", e);
                continue;
            }
        };

        handle_connection(stream).await;
    }
}
