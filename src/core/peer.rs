#[derive(Clone)]
pub struct Peer {
    pub id: String,
    pub public_key: Vec<u8>,
    pub address: String,
}

impl Peer {
    pub fn new(id: String, public_key: Vec<u8>, address: String) -> Self {
        Self { id, public_key, address }
    }
}
