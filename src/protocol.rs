use crate::VDBConnection;
use tokio::io::AsyncReadExt;


impl VDBConnection<'_> {
    pub async fn initialize(&mut self) -> std::io::Result<()> {
        let mut cmd: [u8; 3] = [0; 3];

        match self.io.read_exact(&mut cmd).await {
            Ok(_) => {
                if cmd[0] != 'V' as u8 || cmd[1] != 'D' as u8 || cmd[2] != 'B' as u8 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        format!("Unknown protocol, expected VDB got {}", String::from_utf8_lossy(&cmd))
                    ));
                }
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
