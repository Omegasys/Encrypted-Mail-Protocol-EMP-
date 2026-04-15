pub struct ClientConfig {
    pub server_addr: String,
    pub use_tor: bool,
    pub timeout_ms: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1:9000".to_string(),
            use_tor: false,
            timeout_ms: 5000,
        }
    }
}
