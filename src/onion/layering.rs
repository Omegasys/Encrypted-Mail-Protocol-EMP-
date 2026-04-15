use crate::core::errors::ProtocolError;

pub struct OnionLayer;

impl OnionLayer {
    pub fn encrypt_layers(mut data: Vec<u8>, keys: Vec<Vec<u8>>) -> Vec<u8> {
        for key in keys.into_iter().rev() {
            data = Self::simple_xor(&data, &key);
        }
        data
    }

    pub fn decrypt_layer(data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
        Ok(Self::simple_xor(&data, &key))
    }

    fn simple_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
        data.iter()
            .enumerate()
            .map(|(i, b)| b ^ key[i % key.len()])
            .collect()
    }
}
