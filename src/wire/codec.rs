use crate::wire::frame::Frame;
use crate::core::errors::ProtocolError;

pub struct Codec;

impl Codec {
    pub fn encode(frame: &Frame) -> Vec<u8> {
        frame.to_bytes()
    }

    pub fn decode(data: &[u8]) -> Result<Frame, ProtocolError> {
        Frame::from_bytes(data)
    }
}
