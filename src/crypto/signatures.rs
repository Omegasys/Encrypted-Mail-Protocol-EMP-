use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand_core::OsRng;

pub struct SignatureKeyPair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

impl SignatureKeyPair {
    pub fn generate() -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();

        Self { signing, verifying }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing.sign(message)
    }

    pub fn verify(public: &VerifyingKey, message: &[u8], sig: &Signature) -> bool {
        public.verify(message, sig).is_ok()
    }
}
