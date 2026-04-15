use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use crate::wire::packet::Packet;
use crate::wire::frame::Frame;
use crate::wire::codec::Codec;

pub async fn send_message(addr: &str, message: &str) {
    let mut stream = TcpStream::connect(addr)
        .await
        .expect("Failed to connect");

    let packet = Packet::new(1, 0x02, message.as_bytes().to_vec());
    let frame = Frame::new(packet);

    let data = Codec::encode(&frame);

    stream.write_all(&data).await.unwrap();

    let mut buffer = vec![0u8; 4096];
    let n = stream.read(&mut buffer).await.unwrap();

    println!("Response: {:?}", &buffer[..n]);
}
