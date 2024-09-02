use crate::client::VDBClient;
use crate::VDBConnection;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VDBPeerInfo {
    pub version: String,
    pub app_name: String,
}

pub async fn receive_handshake<'a>(conn: &mut VDBConnection<'a>) -> std::io::Result<()> {
    let mut hdr: [u8; 3] = [0; 3];
    match conn.io.read_exact(&mut hdr).await {
        Ok(_) => {
            if hdr[0] != 'V' as u8 || hdr[1] != 'D' as u8 || hdr[2] != 'B' as u8 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    format!("Unknown protocol, expected VDB got {}", String::from_utf8_lossy(&hdr))
                ));
            }

            let size = conn.io.read_u32().await? as usize;
            let mut client_info = vec![0; size];
            conn.io.read_exact(&mut client_info).await?;

            conn.client_info = match rmp_serde::from_slice(&client_info) {
                Ok(info) => Some(info),
                Err(e) => {
                    return Err(
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Invalid client info: {}", e)
                        )
                    );
                },
            };

            let info = VDBPeerInfo{
                version: env!("CARGO_PKG_VERSION").into(),
                app_name: "VectorDB".into(),
            };

            let serialized = rmp_serde::to_vec(&info).expect("Fatal Error: Failed to serialize client info");

            conn.io.write_u32(serialized.len() as u32).await?;
            conn.io.write_all(&serialized).await?;
            Ok(())
        },
        Err(e) => {Err(e)},
    }
}

pub async fn send_handshake<'a>(client: &mut VDBClient<'a>) -> std::io::Result<()> {
    let mut payload = Vec::new();
    client.info.serialize(&mut Serializer::new(&mut payload).with_binary()).unwrap();

    client.io.write_all("VDB".as_bytes()).await?;
    client.io.write_u32(payload.len() as u32).await?;
    client.io.write_all(&payload).await?;

    match client.io.read_u32().await {
        Ok(size) => {
            let mut response = vec![0; size as usize];
            client.io.read_exact(&mut response).await?;
            client.server_info = match rmp_serde::from_read(&*response) {
                Ok(info) => Some(info),
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Bad server response: Invalid server info: {}", e)
                    ))
                },
            };
            Ok(())
        },
        Err(e) => Err(e),
    }
}
