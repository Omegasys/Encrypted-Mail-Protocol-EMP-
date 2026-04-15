use crate::message::message::Message;
use crate::message::metadata::Metadata;
use crate::crypto::symmetric::SymmetricKey;
use crate::message::envelope::Envelope;

pub struct ClientAPI;

impl ClientAPI {
    pub fn create_message(sender: &str, recipient: &str, body: &str) -> Message {
        Message::new(
            uuid::Uuid::new_v4().to_string(),
            sender.to_string(),
            recipient.to_string(),
            body.as_bytes().to_vec(),
            Metadata::new(),
        )
    }

    pub fn encrypt_message(message: &Message, key: &SymmetricKey) -> Envelope {
        Envelope::encrypt(message, key)
    }
}
