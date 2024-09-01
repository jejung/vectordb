use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct VDBClient<'a> {
    io: &'a mut TcpStream,
}

impl <'a> VDBClient<'a> {

    async fn initialize(&mut self) -> std::io::Result<()> {
        match self.io.write_all("VDB".as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn connect(io: &'a mut TcpStream) -> std::io::Result<Self> {
        let mut x = Self{
            io,
        };

        x.initialize().await?;

        Ok(x)
    }
}
