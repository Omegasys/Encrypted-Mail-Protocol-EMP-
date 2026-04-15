use crate::message::message::Message;
use crate::core::errors::ProtocolError;

pub struct Validator;

impl Validator {
    pub fn validate_message(msg: &Message) -> Result<(), ProtocolError> {
        if msg.id.is_empty() {
            return Err(ProtocolError::InvalidFrame);
        }

        if msg.sender.is_empty() || msg.recipient.is_empty() {
            return Err(ProtocolError::InvalidFrame);
        }

        if msg.body.is_empty() {
            return Err(ProtocolError::InvalidFrame);
        }

        Ok(())
    }

    pub fn validate_size(data: &[u8], max_size: usize) -> Result<(), ProtocolError> {
        if data.len() > max_size {
            return Err(ProtocolError::InvalidFrame);
        }
        Ok(())
    }
}
