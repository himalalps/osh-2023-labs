pub mod pool;

use pool::ThreadPool;
use server::*;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{io, net::TcpListener, sync::mpsc, thread, time::Duration};

const BIND_IP: &str = "0.0.0.0:8000";

fn main() {
    let listener =
        TcpListener::bind(BIND_IP).expect(format!("Failed to bind to {}.", BIND_IP).as_str());
    // 设置非阻塞
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let pool = ThreadPool::new(32);
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    // 用于传送信号
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for _signal in signals.forever() {
            tx.send(()).expect("Could not send signal on channel.");
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(tcpstream) => {
                tcpstream
                    .set_nonblocking(false)
                    .expect("Cannot set blocking");
                pool.execute(|| {
                    handle_connection(tcpstream);
                });
            }
            // 阻塞，说明在等待
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 尝试接收结束信号
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(_) => {
                        // 防止轮询过快对系统占用较高
                        thread::sleep(Duration::from_micros(1));
                    }
                };
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };
    }
}
