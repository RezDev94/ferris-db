use std::fmt;

#[derive(Debug)]
pub enum FerrisError {
    KeyNotFound(String),
    KeyExists(String),
    Persistence(String),
    InvalidCommand(String),
    InvalidTTL(String),
}

impl fmt::Display for FerrisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FerrisError::KeyNotFound(key) => write!(f, "key '{}' not found", key),
            FerrisError::KeyExists(key) => write!(f, "key '{}' already exists", key),
            FerrisError::Persistence(msg) => write!(f, "persistence error: {}", msg),
            FerrisError::InvalidCommand(msg) => write!(f, "invalid command: {}", msg),
            FerrisError::InvalidTTL(msg) => write!(f, "invalid TTL: {}", msg),
        }
    }
}

impl std::error::Error for FerrisError {}

pub type Result<T> = std::result::Result<T, FerrisError>;