use crate::crypto::asymmetric::KeyPair;

/// Ephemeral keys for forward secrecy
pub struct EphemeralSession {
    pub keypair: KeyPair,
}

impl EphemeralSession {
    pub fn new() -> Self {
        Self {
            keypair: KeyPair::generate(),
        }
    }

    pub fn derive_shared(&self, peer_pub: &x25519_dalek::PublicKey) -> [u8; 32] {
        self.keypair.shared_secret(peer_pub)
    }
}
