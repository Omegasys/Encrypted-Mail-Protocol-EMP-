use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::wire::codec::Codec;
use crate::core::errors::ProtocolError;
use crate::utils::logger::{Logger, LogLevel};

pub struct RelayService;

impl RelayService {
    pub async fn handle_connection(mut stream: TcpStream) -> Result<(), ProtocolError> {
        let mut buffer = vec![0u8; 4096];

        loop {
            let n = stream.read(&mut buffer)
                .await
                .map_err(|_| ProtocolError::IoError)?;

            if n == 0 {
                break;
            }

            let data = &buffer[..n];

            let frame = Codec::decode(data)?;

            Logger::log(LogLevel::Debug, "Relaying packet");

            // In real system: decrypt onion layer here

            // Echo forward (placeholder behavior)
            let encoded = Codec::encode(&frame);

            stream.write_all(&encoded)
                .await
                .map_err(|_| ProtocolError::IoError)?;
        }

        Ok(())
    }
}
