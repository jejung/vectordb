use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream};

pub async fn handle_conn(socket: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => {
                println!("Failed to read from socket");
                return Err(e);
            }
        };

        if socket.write_all(&buf[0..n]).await.is_err() {
            println!("Failed to write to socket");
            return Err(std::io::Error::from(std::io::ErrorKind::WriteZero));
        }
    }
}
