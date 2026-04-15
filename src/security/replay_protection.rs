use std::collections::HashSet;

pub struct ReplayProtection {
    seen_ids: HashSet<String>,
}

impl ReplayProtection {
    pub fn new() -> Self {
        Self {
            seen_ids: HashSet::new(),
        }
    }

    pub fn is_replay(&mut self, message_id: &str) -> bool {
        if self.seen_ids.contains(message_id) {
            true
        } else {
            self.seen_ids.insert(message_id.to_string());
            false
        }
    }

    pub fn clear_old(&mut self, max_entries: usize) {
        if self.seen_ids.len() > max_entries {
            self.seen_ids.clear(); // simple reset strategy
        }
    }
}
