use crate::crypto::symmetric::SymmetricKey;
use crate::message::message::Message;

pub struct Envelope {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
}

impl Envelope {
    pub fn encrypt(message: &Message, key: &SymmetricKey) -> Self {
        let serialized = crate::message::serializer::serialize(message);

        let (ciphertext, nonce) = key.encrypt(&serialized);

        Self { ciphertext, nonce }
    }

    pub fn decrypt(&self, key: &SymmetricKey) -> Option<Message> {
        let plaintext = key.decrypt(&self.ciphertext, &self.nonce)?;

        crate::message::serializer::deserialize(&plaintext).ok()
    }
}
