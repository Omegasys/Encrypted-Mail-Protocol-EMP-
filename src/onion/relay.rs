use crate::core::errors::ProtocolError;

pub struct Relay;

impl Relay {
    pub fn forward(data: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
        // Placeholder: decrypt one layer, forward remainder
        Ok(data)
    }
}
