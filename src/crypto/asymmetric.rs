use x25519_dalek::{PublicKey, StaticSecret};
use rand_core::OsRng;

pub struct KeyPair {
    pub private: StaticSecret,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let private = StaticSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&private);

        Self { private, public }
    }

    pub fn shared_secret(&self, peer_public: &PublicKey) -> [u8; 32] {
        self.private.diffie_hellman(peer_public).to_bytes()
    }
}
