use std::collections::HashMap;
use crate::mailbox::mailbox::Mailbox;
use crate::message::message::Message;

pub struct MailboxServer {
    mailboxes: HashMap<String, Mailbox>,
}

impl MailboxServer {
    pub fn new() -> Self {
        Self {
            mailboxes: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, user: &str) -> &mut Mailbox {
        self.mailboxes
            .entry(user.to_string())
            .or_insert_with(|| Mailbox::new(user.to_string()))
    }

    pub fn store_message(&mut self, recipient: &str, message: Message) {
        let mailbox = self.get_or_create(recipient);
        mailbox.add_message(message);
    }

    pub fn retrieve_messages(&self, user: &str) -> Option<&Vec<Message>> {
        self.mailboxes.get(user).map(|m| m.get_all())
    }
}
