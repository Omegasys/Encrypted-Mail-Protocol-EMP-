use crate::core::session::Session;
use crate::core::errors::ProtocolError;

pub struct Protocol {
    pub version: u8,
}

impl Protocol {
    pub fn new(version: u8) -> Self {
        Self { version }
    }

    pub async fn handle_session(&self, session: &mut Session) -> Result<(), ProtocolError> {
        loop {
            let frame = session.read_frame().await?;

            match frame.frame_type {
                0x01 => session.handle_handshake(frame).await?,
                0x02 => session.handle_data(frame).await?,
                0x03 => session.handle_relay(frame).await?,
                0x04 => session.handle_keepalive(frame).await?,
                _ => return Err(ProtocolError::UnknownFrameType),
            }
        }
    }
}
