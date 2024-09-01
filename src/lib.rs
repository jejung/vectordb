use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub mod server;
pub mod protocol;
pub mod client;

pub struct VDBConnection<'a> {
    io: &'a mut TcpStream,
}

impl <'a> VDBConnection<'a> {
    pub fn new(io: &'a mut TcpStream) -> Self {
        Self {
            io,
        }
    }

    pub async fn close(&mut self) -> () {
        self.io.shutdown().await.unwrap_or(());
    }
}
