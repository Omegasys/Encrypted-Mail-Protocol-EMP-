use crate::core::errors::ProtocolError;

#[derive(Debug)]
pub struct Handshake {
    pub version: u8,
    pub public_key: Vec<u8>,
    pub features: Vec<String>,
}

impl Handshake {
    pub fn new(version: u8, public_key: Vec<u8>, features: Vec<String>) -> Self {
        Self {
            version,
            public_key,
            features,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(self.version);
        data.extend(&(self.public_key.len() as u32).to_be_bytes());
        data.extend(&self.public_key);

        data.extend(&(self.features.len() as u32).to_be_bytes());
        for f in &self.features {
            let bytes = f.as_bytes();
            data.extend(&(bytes.len() as u32).to_be_bytes());
            data.extend(bytes);
        }

        data
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, ProtocolError> {
        let mut index = 0;

        let version = data[index];
        index += 1;

        let pk_len = u32::from_be_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        let public_key = data[index..index + pk_len].to_vec();
        index += pk_len;

        let feature_count = u32::from_be_bytes(data[index..index + 4].try_into().unwrap()) as usize;
        index += 4;

        let mut features = Vec::new();
        for _ in 0..feature_count {
            let len = u32::from_be_bytes(data[index..index + 4].try_into().unwrap()) as usize;
            index += 4;

            let f = String::from_utf8(data[index..index + len].to_vec()).unwrap();
            index += len;

            features.push(f);
        }

        Ok(Self {
            version,
            public_key,
            features,
        })
    }
}
