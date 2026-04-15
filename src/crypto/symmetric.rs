use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;

pub struct SymmetricKey {
    pub key: [u8; 32],
}

impl SymmetricKey {
    pub fn generate() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self { key }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> (Vec<u8>, [u8; 12]) {
        let cipher = ChaCha20Poly1305::new(&self.key.into());

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext)
            .expect("encryption failure");

        (ciphertext, nonce_bytes)
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8; 12]) -> Option<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(&self.key.into());
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher.decrypt(nonce, ciphertext).ok()
    }
}
