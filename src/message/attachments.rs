#[derive(Clone)]
pub struct Attachment {
    pub filename: String,
    pub data: Vec<u8>,
    pub mime_type: String,
}

impl Attachment {
    pub fn new(filename: String, data: Vec<u8>, mime_type: String) -> Self {
        Self {
            filename,
            data,
            mime_type,
        }
    }
}
