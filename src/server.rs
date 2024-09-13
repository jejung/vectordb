use crate::protocol::{receive_command, receive_handshake, send_response, VDBCommand, VDBCommandKind, VDBOpResultCode};
use crate::VDBConnection;
use rmpv::Value;

async fn handle_update(conn: &mut VDBConnection<'_>, cmd: &VDBCommand) -> std::io::Result<()> {
    match rmp_serde::from_slice::<Value>(cmd.payload.as_slice()) {
        Ok(_) => send_response(
            conn,
            &VDBOpResultCode::Ok,
            &vec![],
        ).await,
        Err(e) => send_response(
            conn,
            &VDBOpResultCode::InvalidPayload,
            format!("INVALID PAYLOAD: {:?}", e).as_bytes(),
        ).await,
    }
}

async fn handle_insert(conn: &mut VDBConnection<'_>, cmd: &VDBCommand) -> std::io::Result<()> {
    match rmp_serde::from_slice::<Value>(cmd.payload.as_slice()) {
        Ok(_) => send_response(
            conn,
            &VDBOpResultCode::Ok,
            &vec![],
        ).await,
        Err(e) => send_response(
            conn,
            &VDBOpResultCode::InvalidPayload,
            format!("INVALID PAYLOAD: {:?}", e).as_bytes(),
        ).await,
    }
}

pub async fn handle_conn(conn: &mut VDBConnection<'_>) -> std::io::Result<()> {
    receive_handshake(conn).await?;
    println!("Client connected: {:?}", conn.client_info.as_ref().unwrap());
    loop {
        match receive_command(conn).await {
            Ok(command) => {
                match command.kind {
                    VDBCommandKind::PING => send_response(
                        conn,
                        &VDBOpResultCode::Ok,
                        b"PONG"
                    ).await?,
                    VDBCommandKind::DISCONNECT => break,
                    VDBCommandKind::INSERT => handle_insert(conn, &command).await?,
                    VDBCommandKind::UPDATE => handle_update(conn, &command).await?,
                    VDBCommandKind::UNKNOWN => send_response(
                        conn,
                        &VDBOpResultCode::UnknownCommand,
                        format!("INVALID COMMAND: {:?}", command.kind).as_bytes(),
                    ).await?
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => break,
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionAborted => break,
            Err(e) if e.kind() == std::io::ErrorKind::ConnectionReset => break,
            Err(e) if e.kind() == std::io::ErrorKind::NotConnected => break,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                println!("Error receiving command: {:?}", e);
                send_response(
                    conn,
                    &VDBOpResultCode::InvalidPayload,
                    format!("INVALID COMMAND: {:?}", e).as_bytes(),
                ).await?;
            },
        }
    }
    println!("Client disconnected: {:?}", conn.client_info.as_ref().unwrap());
    Ok(())
}
