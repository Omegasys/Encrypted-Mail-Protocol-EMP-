use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9000";

    println!("Connecting to {}", addr);

    let mut stream = TcpStream::connect(addr)
        .await
        .expect("Failed to connect");

    let test_data = b"EMP DEBUG TEST";

    println!("Sending: {:?}", test_data);

    stream.write_all(test_data).await.unwrap();

    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await.unwrap();

    println!("Received ({} bytes): {:?}", n, &buf[..n]);
}
