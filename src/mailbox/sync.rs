use crate::message::message::Message;

pub struct SyncState {
    pub last_sync_timestamp: u64,
}

impl SyncState {
    pub fn new() -> Self {
        Self {
            last_sync_timestamp: 0,
        }
    }
}

pub struct SyncEngine;

impl SyncEngine {
    pub fn get_updates(messages: &[Message], state: &SyncState) -> Vec<Message> {
        messages
            .iter()
            .filter(|m| m.metadata.timestamp > state.last_sync_timestamp)
            .cloned()
            .collect()
    }

    pub fn update_state(state: &mut SyncState, messages: &[Message]) {
        if let Some(max_ts) = messages.iter().map(|m| m.metadata.timestamp).max() {
            state.last_sync_timestamp = max_ts;
        }
    }
}
