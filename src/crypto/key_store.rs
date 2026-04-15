use std::collections::HashMap;

pub struct KeyStore {
    keys: HashMap<String, Vec<u8>>,
}

impl KeyStore {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn store(&mut self, id: &str, key: Vec<u8>) {
        self.keys.insert(id.to_string(), key);
    }

    pub fn get(&self, id: &str) -> Option<&Vec<u8>> {
        self.keys.get(id)
    }

    pub fn remove(&mut self, id: &str) {
        self.keys.remove(id);
    }
}
