use crate::message::metadata::Metadata;

#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub body: Vec<u8>,
    pub metadata: Metadata,
}

impl Message {
    pub fn new(id: String, sender: String, recipient: String, body: Vec<u8>, metadata: Metadata) -> Self {
        Self {
            id,
            sender,
            recipient,
            body,
            metadata,
        }
    }
}
