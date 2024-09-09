use crate::client::VDBAsyncClient;
use crate::VDBConnection;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct VDBPeerInfo {
    pub version: String,
    pub app_name: String,
}

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, FromPrimitive, ToPrimitive)]
pub enum VDBCommandKind {
    UNKNOWN = 0,
    INSERT = 1,
    PING = 5,
    DISCONNECT = 6,
}

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, FromPrimitive, ToPrimitive)]
pub enum VDBOpResultCode {
    Ok = 0,
    InvalidPayload = 1,
    CommandNotImplemented = 2,
    UnknownCommand = 3,
}

pub struct VDBCommand {
    pub kind: VDBCommandKind,
    pub payload: Vec<u8>,
}

async fn command_carry_payload(kind: u8) -> bool {
    kind != VDBCommandKind::PING.to_u8().unwrap()
        && kind != VDBCommandKind::DISCONNECT.to_u8().unwrap()
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

pub async fn receive_command(conn: &mut VDBConnection<'_>) -> std::io::Result<VDBCommand> {
    let kind = VDBCommandKind::from_u8(conn.io.read_u8().await?).unwrap_or(VDBCommandKind::UNKNOWN);

    if !command_carry_payload(kind.to_u8().unwrap()).await {
        return Ok(
            VDBCommand{
                kind,
                payload: vec![],
            }
        );
    }

    let payload_size = conn.io.read_u32().await?;
    let mut buf = vec![0; payload_size as usize];
    conn.io.read_exact(&mut buf).await?;
    Ok(
        VDBCommand{
            kind,
            payload: buf,
        }
    )
}

pub async fn send_response(conn: &mut VDBConnection<'_>, result_code: &VDBOpResultCode, payload: &[u8]) -> std::io::Result<()> {
    conn.io.write_u16(result_code.to_u16().unwrap()).await?;
    conn.io.write_u32(payload.len() as u32).await?;
    conn.io.write_all(payload).await?;
    Ok(())
}

pub async fn send_handshake<'a>(client: &mut VDBAsyncClient<'a>) -> std::io::Result<()> {
    let mut payload = Vec::new();
    match client.info.serialize(&mut Serializer::new(&mut payload).with_binary()) {
        Ok(()) => {},
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Could not serialize client info: {}", e)
            ));
        }
    };

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

pub async fn send_command(conn: &mut VDBAsyncClient<'_>, command: &VDBCommand) -> std::io::Result<()> {
    conn.io.write_u8(command.kind.to_u8().unwrap()).await?;
    if command_carry_payload(command.kind.to_u8().unwrap()).await {
        conn.io.write_u32(command.payload.len() as u32).await?;
        conn.io.write_all(command.payload.as_slice()).await?;
    }
    Ok(())
}

pub async fn receive_response(conn: &mut VDBAsyncClient<'_>) -> std::io::Result<Vec<u8>> {
    let result_code = conn.io.read_u16().await?;
    let size = conn.io.read_u32().await?;
    let mut payload = vec![0; size as usize];
    conn.io.read_exact(payload.as_mut_slice()).await?;

    if result_code == 0 {
        return Ok(payload);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        String::from_utf8_lossy(&payload).to_string(),
    ))
}
