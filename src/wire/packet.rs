use crate::core::errors::ProtocolError;

pub const MAGIC: u16 = 0x454D; // "EM"

#[derive(Debug, Clone)]
pub struct Packet {
    pub version: u8,
    pub packet_type: u8,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(version: u8, packet_type: u8, payload: Vec<u8>) -> Self {
        Self {
            version,
            packet_type,
            payload,
        }
    }

    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.payload.is_empty() {
            return Err(ProtocolError::InvalidFrame);
        }
        Ok(())
    }
}
