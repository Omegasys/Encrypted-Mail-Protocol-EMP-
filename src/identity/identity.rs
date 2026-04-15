use crate::crypto::signatures::SignatureKeyPair;

#[derive(Clone)]
pub struct Identity {
    pub id: String,
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl Identity {
    pub fn generate() -> Self {
        let keypair = SignatureKeyPair::generate();

        let public_key = keypair.verifying.to_bytes().to_vec();
        let private_key = keypair.signing.to_bytes().to_vec();

        let id = hex::encode(&public_key[..8]); // short ID

        Self {
            id,
            public_key,
            private_key,
        }
    }
}
