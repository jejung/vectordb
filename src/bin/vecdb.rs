use std::process::exit;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;

#[tokio::main]
async fn main() -> () {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    let mut stream = match socket.connect(addr).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            return;
        }
    };

    println!("Connected VectorDB running@127.0.0.1:9999");
    println!("Sending hello world");

    match stream.write(b"hello world").await {
        Err(e) => panic!("{}", e),
        Ok(len) => println!("Wrote {} bytes", len),
    };

    let mut buf = [0; 1024];
    match stream.read(&mut buf).await {
        Err(_) => {
            println!("Failed to read from socket");
            exit(1);
        },
        Ok(n) => println!("Received: {}", String::from_utf8_lossy(&buf[..n])),
    };
}
