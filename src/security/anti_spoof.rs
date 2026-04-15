use crate::crypto::signatures::{SignatureKeyPair};
use ed25519_dalek::{Signature, VerifyingKey};

pub struct AntiSpoof;

impl AntiSpoof {
    pub fn sign_message(keypair: &SignatureKeyPair, message: &[u8]) -> Signature {
        keypair.sign(message)
    }

    pub fn verify_message(
        public_key_bytes: &[u8],
        message: &[u8],
        signature: &Signature,
    ) -> bool {
        if let Ok(pubkey) = VerifyingKey::from_bytes(public_key_bytes.try_into().unwrap()) {
            SignatureKeyPair::verify(&pubkey, message, signature)
        } else {
            false
        }
    }
}
