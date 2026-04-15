use crate::wire::packet::Packet;
use crate::core::errors::ProtocolError;

#[derive(Debug)]
pub struct Frame {
    pub magic: u16,
    pub packet: Packet,
}

impl Frame {
    pub fn new(packet: Packet) -> Self {
        Self {
            magic: super::packet::MAGIC,
            packet,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(&self.magic.to_be_bytes());
        bytes.push(self.packet.version);
        bytes.push(self.packet.packet_type);

        let len = self.packet.payload.len() as u32;
        bytes.extend(&len.to_be_bytes());

        bytes.extend(&self.packet.payload);

        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::InvalidFrame);
        }

        let magic = u16::from_be_bytes([data[0], data[1]]);
        if magic != super::packet::MAGIC {
            return Err(ProtocolError::InvalidFrame);
        }

        let version = data[2];
        let packet_type = data[3];
        let len = u32::from_be_bytes([data[4], data[5], data[6], data[7]]) as usize;

        if data.len() < 8 + len {
            return Err(ProtocolError::InvalidFrame);
        }

        let payload = data[8..8 + len].to_vec();

        Ok(Frame {
            magic,
            packet: Packet::new(version, packet_type, payload),
        })
    }
}
