use crate::crypto::asymmetric::KeyPair;
use sha2::{Sha256, Digest};

pub struct HandshakeState {
    pub local: KeyPair,
    pub shared_secret: Option<[u8; 32]>,
}

impl HandshakeState {
    pub fn new() -> Self {
        Self {
            local: KeyPair::generate(),
            shared_secret: None,
        }
    }

    pub fn initiate(&self) -> [u8; 32] {
        self.local.public.to_bytes()
    }

    pub fn respond(&mut self, peer_pub_bytes: [u8; 32]) -> [u8; 32] {
        let peer_pub = x25519_dalek::PublicKey::from(peer_pub_bytes);

        let shared = self.local.shared_secret(&peer_pub);
        self.shared_secret = Some(shared);

        self.local.public.to_bytes()
    }

    pub fn finalize(&mut self, peer_pub_bytes: [u8; 32]) {
        let peer_pub = x25519_dalek::PublicKey::from(peer_pub_bytes);
        let shared = self.local.shared_secret(&peer_pub);
        self.shared_secret = Some(shared);
    }

    pub fn derive_session_key(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.shared_secret.unwrap());
        hasher.finalize().into()
    }
}
