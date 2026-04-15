use std::collections::HashMap;
use crate::message::message::Message;

pub struct MailIndex {
    by_id: HashMap<String, Message>,
    by_sender: HashMap<String, Vec<String>>,
}

impl MailIndex {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_sender: HashMap::new(),
        }
    }

    pub fn index_message(&mut self, message: Message) {
        let id = message.id.clone();
        let sender = message.sender.clone();

        self.by_id.insert(id.clone(), message);

        self.by_sender
            .entry(sender)
            .or_insert_with(Vec::new)
            .push(id);
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Message> {
        self.by_id.get(id)
    }

    pub fn get_by_sender(&self, sender: &str) -> Vec<&Message> {
        match self.by_sender.get(sender) {
            Some(ids) => ids.iter().filter_map(|id| self.by_id.get(id)).collect(),
            None => Vec::new(),
        }
    }
}
