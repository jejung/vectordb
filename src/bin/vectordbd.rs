use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use vectordb::server::handle_conn;
use vectordb::VDBConnection;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Server running on 127.0.0.1:9999");

    loop {
        let (mut socket, _): (TcpStream, SocketAddr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut conn = VDBConnection::new(&mut socket);
            println!("Received connection, handling.");
            if let Err(e) = handle_conn(&mut conn).await {
                println!("Error handling connection: {}", e);
                conn.close().await;
                return;
            }
        });
    }
}
