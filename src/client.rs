use crate::datastructures::Document;
use crate::protocol::VDBPeerInfo;
use crate::protocol::{receive_response, send_command, send_handshake, VDBCommand, VDBCommandKind};
use tokio::net::TcpStream;

pub struct VDBAsyncClient<'a> {
    pub(crate) io: &'a mut TcpStream,
    pub(crate) info: VDBPeerInfo,
    pub server_info: Option<VDBPeerInfo>,
}

pub struct VDBPingResponse {
    pub success: bool,
    pub content: String,
    pub error: Option<String>,
}

pub struct VDBInsertResponse {
    pub success: bool,
    pub error: Option<String>,
}

impl <'a> VDBAsyncClient<'a> {

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

    pub async fn ping(&mut self) -> std::io::Result<VDBPingResponse> {
        send_command(self, &VDBCommand{
            kind: VDBCommandKind::PING,
            payload: vec![],
        }).await?;

        match receive_response(self).await {
            Ok(payload) => Ok(VDBPingResponse{
                success: true,
                content: String::from_utf8_lossy(&payload).to_string(),
                error: None,
            }),
            Err(e) => Ok(VDBPingResponse{
                success: false,
                content: String::new(),
                error: Some(e.to_string()),
            }),
        }
    }

    pub async fn insert(&mut self, data: &Vec<Document>) -> std::io::Result<VDBInsertResponse> {
        match rmp_serde::to_vec_named(&data) {
            Ok(payload) => {
                send_command(self, &VDBCommand {
                    kind: VDBCommandKind::INSERT,
                    payload,
                }).await?;

                match receive_response(self).await {
                    Ok(_) => Ok(VDBInsertResponse{success: true, error: None}),
                    Err(e) => Ok(VDBInsertResponse{success: false, error: Some(e.to_string())}),
                }
            },
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Could not serialize documents {:?}", e),
            )),
        }
    }

    pub async fn disconnect(&mut self) -> std::io::Result<()> {
        send_command(self, &VDBCommand{
            kind: VDBCommandKind::DISCONNECT,
            payload: vec![],
        }).await
    }
}
