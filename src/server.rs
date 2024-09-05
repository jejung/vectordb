use crate::protocol::{receive_command, receive_handshake, send_response, VDBCommandKind};
use crate::VDBConnection;

pub async fn handle_conn(conn: &mut VDBConnection<'_>) -> std::io::Result<()> {
    receive_handshake(conn).await?;
    println!("Client connected: {:?}", conn.client_info.as_ref().unwrap());
    loop {
        match receive_command(conn).await {
            Ok(command) => {
                match command.kind {
                    VDBCommandKind::PING => send_response(conn, 0, b"PONG").await?,
                    VDBCommandKind::DISCONNECT => break,
                    _=> send_response(conn, 2, format!("COMMAND NOT IMPLEMENTED: {:?}", command.kind).as_bytes()).await?
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => break,
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionAborted => break,
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionReset => break,
            Err(e) if e.kind() == std::io::ErrorKind::NotConnected => break,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                println!("Error receiving command: {:?}", e);
                send_response(conn, 1, format!("INVALID COMMAND: {:?}", e).as_bytes()).await?;
            },
        }
    }
    println!("Client disconnected: {:?}", conn.client_info.as_ref().unwrap());
    Ok(())
}
