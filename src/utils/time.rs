use std::time::{SystemTime, UNIX_EPOCH};

pub struct Time;

impl Time {
    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn expired(timestamp: u64, ttl: u64) -> bool {
        let now = Self::now();
        now > timestamp + ttl
    }

    pub fn delay_ms(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}
