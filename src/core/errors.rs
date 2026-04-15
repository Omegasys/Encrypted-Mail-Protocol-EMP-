use std::fmt;

#[derive(Debug)]
pub enum ProtocolError {
    IoError,
    InvalidFrame,
    UnknownFrameType,
    CryptoError,
    NotImplemented,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::IoError => write!(f, "I/O error"),
            ProtocolError::InvalidFrame => write!(f, "Invalid frame"),
            ProtocolError::UnknownFrameType => write!(f, "Unknown frame type"),
            ProtocolError::CryptoError => write!(f, "Cryptographic error"),
            ProtocolError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl std::error::Error for ProtocolError {}
