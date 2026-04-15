use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::core::errors::ProtocolError;

pub struct TcpTransport;

impl TcpTransport {
    pub async fn connect(addr: &str) -> Result<TcpStream, ProtocolError> {
        TcpStream::connect(addr)
            .await
            .map_err(|_| ProtocolError::IoError)
    }

    pub async fn listen(addr: &str) -> Result<TcpListener, ProtocolError> {
        TcpListener::bind(addr)
            .await
            .map_err(|_| ProtocolError::IoError)
    }

    pub async fn send(stream: &mut TcpStream, data: &[u8]) -> Result<(), ProtocolError> {
        stream.write_all(data)
            .await
            .map_err(|_| ProtocolError::IoError)
    }

    pub async fn receive(stream: &mut TcpStream) -> Result<Vec<u8>, ProtocolError> {
        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf)
            .await
            .map_err(|_| ProtocolError::IoError)?;

        buf.truncate(n);
        Ok(buf)
    }
}
