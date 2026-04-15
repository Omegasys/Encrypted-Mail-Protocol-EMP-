use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Metadata {
    pub timestamp: u64,
    pub ttl: Option<u64>,
    pub flags: u32,
}

impl Metadata {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            timestamp,
            ttl: None,
            flags: 0,
        }
    }
}
