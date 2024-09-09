use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;
use vectordb::client::VDBAsyncClient;

async fn report_msg(msg: String) {
    let mut stdout = tokio::io::stdout();
    if msg.len() > 0 {
        let _ = stdout.write_all(msg.as_bytes()).await;
        let _ = stdout.write_all(b"\n").await;
    }
    if atty::is(atty::Stream::Stdin) {
        let _ = stdout.write_all(b">> ").await;
    }
    let _ = stdout.flush().await;
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    let mut stream = match socket.connect(addr).await {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to server: {}", e);
            return Err(e);
        }
    };
    let mut vdb = VDBAsyncClient::connect(&mut stream).await?;
    println!("Connected to VDB server: {:?}", vdb.server_info.as_ref().unwrap());

    let stdin = tokio::io::stdin();
    let reader = tokio::io::BufReader::new(stdin);
    let mut lines = reader.lines();

    report_msg(String::new()).await;

    while let Some(line) = lines.next_line().await?  {
        let mut command_and_rest = line.split_whitespace();
        match command_and_rest.next() {
            None => (),
            Some(command) => match command {
                "INS" => {
                    let mut all_documents = "".to_string();
                    while let Some(arg) = command_and_rest.next()  {
                        all_documents.push_str(&arg);
                    }
                    match all_documents.as_str() {
                        "" => {
                            report_msg("Wrong number of arguments, expected DOCUMENTS.".to_string()).await;
                            continue;
                        },
                        documents => {
                            match serde_json::from_str(documents) {
                                Ok(documents) => {
                                    match vdb.insert(&documents).await {
                                        Ok(response) => {
                                            match response.success {
                                                true => report_msg(format!("{} documents inserted.", documents.len())).await,
                                                false => report_msg(format!("Failed to insert documents: {}", response.error.unwrap_or(String::new()))).await,
                                            }
                                            continue;
                                        },
                                        Err(e) => {
                                            report_msg(format!("Could not insert documents: {}", e)).await;
                                            continue;
                                        }
                                    }
                                }
                                Err(e) => {
                                    report_msg(format!("Could not parse documents, invalid JSON: {}", e)).await;
                                    continue;
                                }
                            }
                        }
                    }
                },
                "PING" => {
                    if let Some(arg) = command_and_rest.next() {
                        report_msg(format!("Expected no arguments, got: {}", arg)).await;
                        continue;
                    }
                    match vdb.ping().await {
                        Ok(response) => {
                            report_msg(response.content).await;
                            continue;
                        },
                        Err(e) => {
                            report_msg(format!("Failed to ping: {}", e)).await;
                            continue;
                        },
                    }
                },
                "EXIT" => break,
                "" => (),
                unknown_command => {
                    report_msg(format!("Unknown command: {}", unknown_command)).await;
                    continue;
                },
            },
        }
        report_msg(String::new()).await;
    }

    Ok(())
}
