pub struct SecurityPolicy {
    pub require_encryption: bool,
    pub require_signature: bool,
    pub max_message_size: usize,
    pub allow_unsigned_local: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            require_encryption: true,
            require_signature: true,
            max_message_size: 1024 * 1024, // 1 MB
            allow_unsigned_local: false,
        }
    }
}

impl SecurityPolicy {
    pub fn is_valid_size(&self, size: usize) -> bool {
        size <= self.max_message_size
    }
}
