use sha2::{Sha256, Digest};

pub struct Address;

impl Address {
    pub fn from_public_key(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();

        format!("emp://{}", hex::encode(&hash[..16]))
    }
}
