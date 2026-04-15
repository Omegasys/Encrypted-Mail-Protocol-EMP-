use crate::message::message::Message;
use crate::message::metadata::Metadata;
use crate::core::errors::ProtocolError;

pub fn serialize(msg: &Message) -> Vec<u8> {
    let mut data = Vec::new();

    write_string(&mut data, &msg.id);
    write_string(&mut data, &msg.sender);
    write_string(&mut data, &msg.recipient);

    data.extend(&(msg.body.len() as u32).to_be_bytes());
    data.extend(&msg.body);

    data.extend(&msg.metadata.timestamp.to_be_bytes());

    data
}

pub fn deserialize(data: &[u8]) -> Result<Message, ProtocolError> {
    let mut index = 0;

    let id = read_string(data, &mut index)?;
    let sender = read_string(data, &mut index)?;
    let recipient = read_string(data, &mut index)?;

    let body_len = read_u32(data, &mut index)? as usize;
    let body = data[index..index + body_len].to_vec();
    index += body_len;

    let timestamp = read_u64(data, &mut index)?;

    let metadata = Metadata {
        timestamp,
        ttl: None,
        flags: 0,
    };

    Ok(Message {
        id,
        sender,
        recipient,
        body,
        metadata,
    })
}

fn write_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    buf.extend(&(bytes.len() as u32).to_be_bytes());
    buf.extend(bytes);
}

fn read_string(data: &[u8], index: &mut usize) -> Result<String, ProtocolError> {
    let len = read_u32(data, index)? as usize;
    let s = String::from_utf8(data[*index..*index + len].to_vec())
        .map_err(|_| ProtocolError::InvalidFrame)?;
    *index += len;
    Ok(s)
}

fn read_u32(data: &[u8], index: &mut usize) -> Result<u32, ProtocolError> {
    if *index + 4 > data.len() {
        return Err(ProtocolError::InvalidFrame);
    }
    let val = u32::from_be_bytes(data[*index..*index + 4].try_into().unwrap());
    *index += 4;
    Ok(val)
}

fn read_u64(data: &[u8], index: &mut usize) -> Result<u64, ProtocolError> {
    if *index + 8 > data.len() {
        return Err(ProtocolError::InvalidFrame);
    }
    let val = u64::from_be_bytes(data[*index..*index + 8].try_into().unwrap());
    *index += 8;
    Ok(val)
}
