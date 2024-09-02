use tokio::net::TcpSocket;
use vectordb::client::VDBClient;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    let mut stream = match socket.connect(addr).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to server: {}", e);
            return Err(e);
        }
    };
    let vdb = VDBClient::connect(&mut stream).await?;
    println!("Connected to VDB server: {:?}", vdb.server_info.unwrap());
    Ok(())
}
