use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[test]
fn test_server_accepts_connections() {
    let mut server = Command::new("target/debug/vectordbd")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start server");

    let cli_command = Command::new("target/debug/vecdb")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not start cli");

    let commands = b"\
    PING\n\
    INS [{\"vectors\": {}, \"fields\": []}]\n\
    UPD [{\"id\":\"efdsaa\",\"vectors\": {}, \"fields\": []}]\n\
    DEL efdsaa
    ";
    cli_command.stdin
        .expect("Could not connect to cli pipe")
        .write_all(commands)
        .expect("Could not write to cli pipe");

    let mut current_client_output: Vec<u8> = Vec::new();
    cli_command
        .stdout
        .expect("Could not connect to cli output").read_to_end(&mut current_client_output)
        .expect("Could not read from cli output");

    server.kill().unwrap();
    let server_output = server.wait_with_output().unwrap();

    let expected_client_output = "\
    Connected to VDB server: VDBPeerInfo { version: \"2024.9.1\", app_name: \"VectorDB\" }\n\
    PONG\n\
    1 documents inserted.\n\
    1 documents updated.\n\
    1 documents deleted.\n\
    ";
    assert_eq!(String::from_utf8_lossy(&current_client_output), expected_client_output);

    let current_server_output = String::from_utf8_lossy(&server_output.stdout);
    let expected_server_output = "\
    Server running on 127.0.0.1:9999\n\
    Client connected: VDBPeerInfo { version: \"2024.9.1\", app_name: \"vectordb SDK\" }\n\
    Client disconnected: VDBPeerInfo { version: \"2024.9.1\", app_name: \"vectordb SDK\" }\
    ";

    assert_eq!(current_server_output.trim(), expected_server_output);
}
