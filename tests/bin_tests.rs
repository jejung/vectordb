use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_server_accepts_connections() {
    let mut server = Command::new("target/debug/vectordbd")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start server");

    sleep(Duration::from_secs(1));

    let client_output = Command::new("target/debug/vecdb")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Client failed");

    server.kill().unwrap();
    let server_output = server.wait_with_output().unwrap();

    let current_client_output = String::from_utf8_lossy(&client_output.stdout);
    let expected_client_output = "\
    Connected to VDB server: VDBPeerInfo { version: \"2024.9.1\", app_name: \"VectorDB\" }\n\
    \"PONG\"\
    ";
    assert_eq!(current_client_output.trim(), expected_client_output);

    let current_server_output = String::from_utf8_lossy(&server_output.stdout);
    let expected_server_output = "\
    Server running on 127.0.0.1:9999\n\
    Client connected: VDBPeerInfo { version: \"2024.9.1\", app_name: \"vectordb SDK\" }\n\
    Client disconnected: VDBPeerInfo { version: \"2024.9.1\", app_name: \"vectordb SDK\" }\
    ";

    assert_eq!(current_server_output.trim(), expected_server_output);
}
