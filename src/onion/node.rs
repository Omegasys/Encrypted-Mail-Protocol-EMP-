#[derive(Clone)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub public_key: Vec<u8>,
}

impl Node {
    pub fn new(id: String, address: String, public_key: Vec<u8>) -> Self {
        Self {
            id,
            address,
            public_key,
        }
    }
}
