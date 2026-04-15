use tokio::net::{TcpListener, TcpStream};
use crate::core::errors::ProtocolError;

pub struct P2PTransport;

impl P2PTransport {
    pub async fn connect(peer_addr: &str) -> Result<TcpStream, ProtocolError> {
        TcpStream::connect(peer_addr)
            .await
            .map_err(|_| ProtocolError::IoError)
    }

    pub async fn listen(addr: &str) -> Result<TcpListener, ProtocolError> {
        TcpListener::bind(addr)
            .await
            .map_err(|_| ProtocolError::IoError)
    }

    pub async fn accept(listener: &TcpListener) -> Result<(TcpStream, String), ProtocolError> {
        let (stream, addr) = listener.accept()
            .await
            .map_err(|_| ProtocolError::IoError)?;

        Ok((stream, addr.to_string()))
    }
}
