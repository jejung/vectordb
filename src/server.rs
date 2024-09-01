use crate::VDBConnection;

pub async fn handle_conn(conn: &mut VDBConnection<'_>) -> std::io::Result<()> {
    match conn.initialize().await {
        Ok(_) => {},
        Err(e) => {
            println!("Handshake failure: {}", e);
            return Err(e);
        }
    }
    loop {

    }
}
