use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::protocol::VDBPeerInfo;

pub mod server;
pub mod protocol;
pub mod client;

pub struct VDBConnection<'a> {
    pub(crate) io: &'a mut TcpStream,
    pub(crate) client_info: Option<VDBPeerInfo>,
}

impl <'a> VDBConnection<'a> {
    pub fn new(io: &'a mut TcpStream) -> Self {
        Self {
            io,
            client_info: None,
        }
    }

    pub async fn close(&mut self) -> () {
        self.io.shutdown().await.unwrap_or(());
    }
}
