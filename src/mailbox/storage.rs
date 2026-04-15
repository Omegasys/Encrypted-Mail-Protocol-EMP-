use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::message::message::Message;
use crate::message::serializer;

pub struct Storage;

impl Storage {
    pub fn save(path: &str, messages: &[Message]) -> std::io::Result<()> {
        let mut file = File::create(path)?;

        for msg in messages {
            let data = serializer::serialize(msg);
            let len = data.len() as u32;

            file.write_all(&len.to_be_bytes())?;
            file.write_all(&data)?;
        }

        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Vec<Message>> {
        let mut file = OpenOptions::new().read(true).open(path)?;
        let mut messages = Vec::new();

        loop {
            let mut len_buf = [0u8; 4];

            if file.read_exact(&mut len_buf).is_err() {
                break;
            }

            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];

            file.read_exact(&mut buf)?;

            if let Ok(msg) = serializer::deserialize(&buf) {
                messages.push(msg);
            }
        }

        Ok(messages)
    }
}
