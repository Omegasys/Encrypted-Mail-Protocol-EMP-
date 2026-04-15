use crate::server::mailbox_server::MailboxServer;
use crate::message::message::Message;
use crate::security::validation::Validator;
use crate::core::errors::ProtocolError;

pub struct DeliveryService;

impl DeliveryService {
    pub fn deliver(
        server: &mut MailboxServer,
        message: Message,
    ) -> Result<(), ProtocolError> {

        // Validate message before delivery
        Validator::validate_message(&message)?;

        let recipient = message.recipient.clone();

        server.store_message(&recipient, message);

        Ok(())
    }
}
