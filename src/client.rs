use crate::protocol::send_handshake;
use crate::protocol::VDBPeerInfo;
use tokio::net::TcpStream;

pub struct VDBClient<'a> {
    pub(crate) io: &'a mut TcpStream,
    pub(crate) info: VDBPeerInfo,
    pub server_info: Option<VDBPeerInfo>,
}

impl <'a> VDBClient<'a> {

    pub async fn connect(io: &'a mut TcpStream) -> std::io::Result<Self> {
        let mut x = Self{
            io,
            info: VDBPeerInfo{
                version: env!("CARGO_PKG_VERSION").into(),
                app_name: format!("{} SDK", env!("CARGO_PKG_NAME")),
            },
            server_info: None,
        };

        send_handshake(&mut x).await?;

        Ok(x)
    }
}
