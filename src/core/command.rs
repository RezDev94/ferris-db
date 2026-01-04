use super::error::{FerrisError, Result};

#[derive(Debug)]
pub enum Command {
    PING,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        ttl: Option<u64>,
    },
    Delete {
        key: String,
    },
    Rename {
        old_key: String,
        new_key: String,
    },
    Expire {
        key: String,
        ttl: u64,
    },
    TTL {
        key: String,
    },
    Keys,
    Count,
    Clear,
}

impl Command {
    pub fn parse(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.trim().splitn(4, ' ').collect();

        match parts.as_slice() {
            ["PING"] => Ok(Command::PING),
            ["GET", key] => Ok(Command::Get {
                key: key.to_string(),
            }),
            ["SET", key, value, ttl] => {
                let ttl = ttl
                    .parse::<u64>()
                    .map_err(|_| FerrisError::InvalidTTL(ttl.to_string()))?;
                Ok(Command::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                    ttl: Some(ttl),
                })
            }
            ["SET", key, value] => Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
                ttl: None,
            }),
            ["DELETE", key] => Ok(Command::Delete {
                key: key.to_string(),
            }),
            ["RENAME", old_key, new_key] => Ok(Command::Rename {
                old_key: old_key.to_string(),
                new_key: new_key.to_string(),
            }),
            ["EXPIRE", key, ttl] => {
                let ttl = ttl
                    .parse::<u64>()
                    .map_err(|_| FerrisError::InvalidTTL(ttl.to_string()))?;
                Ok(Command::Expire {
                    key: key.to_string(),
                    ttl: ttl,
                })
            }
            ["TTL", key] => Ok(Command::TTL {
                key: key.to_string(),
            }),
            ["KEYS"] => Ok(Command::Keys),
            ["COUNT"] => Ok(Command::Count),
            ["CLEAR"] => Ok(Command::Clear),
            _ => Err(FerrisError::InvalidCommand(input.trim().to_string())),
        }
    }
}
