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

    let mut output = Command::new("target/debug/vecdb")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Client failed");

    let current = String::from_utf8_lossy(&output.stdout);

    let expected = "Connected VectorDB running@127.0.0.1:9999\n\
    Sending hello world\n\
    Wrote 11 bytes\n\
    Received: hello world";
    assert_eq!(current.trim(), expected);

    server.kill().unwrap();
}
