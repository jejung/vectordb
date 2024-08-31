use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use vectordb::server::handle_conn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Server running on 127.0.0.1:9999");

    loop {
        let (mut socket, _): (TcpStream, SocketAddr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_conn(&mut socket).await {
                println!("Error handling connection: {}", e);
            }
        });
    }
}
