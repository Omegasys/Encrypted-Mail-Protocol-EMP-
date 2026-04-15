use crate::message::message::Message;

pub struct Mailbox {
    pub owner: String,
    messages: Vec<Message>,
}

impl Mailbox {
    pub fn new(owner: String) -> Self {
        Self {
            owner,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_all(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.id == id)
    }

    pub fn delete(&mut self, id: &str) {
        self.messages.retain(|m| m.id != id);
    }
}
