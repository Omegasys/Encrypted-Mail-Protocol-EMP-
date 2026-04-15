use crate::core::errors::ProtocolError;

pub struct Frame {
    pub frame_type: u8,
    pub payload: Vec<u8>,
}

pub struct Session {}

impl Session {
    pub async fn read_frame(&mut self) -> Result<Frame, ProtocolError> {
        // Placeholder: implement socket read + decode
        Err(ProtocolError::NotImplemented)
    }

    pub async fn handle_handshake(&mut self, _frame: Frame) -> Result<(), ProtocolError> {
        Ok(())
    }

    pub async fn handle_data(&mut self, _frame: Frame) -> Result<(), ProtocolError> {
        Ok(())
    }

    pub async fn handle_relay(&mut self, _frame: Frame) -> Result<(), ProtocolError> {
        Ok(())
    }

    pub async fn handle_keepalive(&mut self, _frame: Frame) -> Result<(), ProtocolError> {
        Ok(())
    }
}
