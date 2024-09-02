use crate::protocol::receive_handshake;
use crate::VDBConnection;

pub async fn handle_conn(conn: &mut VDBConnection<'_>) -> std::io::Result<()> {
    receive_handshake(conn).await?;
    println!("Client connected: {:?}", conn.client_info.as_ref().unwrap());
    loop {

    }
}
