pub enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        ttl: u64,
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
    Exit,
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().splitn(4, ' ').collect();

        match parts.as_slice() {
            ["GET", key] => Command::Get {
                key: key.to_string(),
            },
            ["SET", key, value, ttl] => {
                let ttl = ttl.parse::<u64>();
                match ttl {
                    Ok(ttl_from_input) => Command::Set {
                        key: key.to_string(),
                        value: value.to_string(),
                        ttl: ttl_from_input,
                    },
                    Err(_) => Command::Unknown(format!("Invalid TTL: {:?}", ttl)),
                }
            }
            ["DELETE", key] => Command::Delete {
                key: key.to_string(),
            },
            ["RENAME", old_key, new_key] => Command::Rename {
                old_key: old_key.to_string(),
                new_key: new_key.to_string(),
            },
            ["EXPIRE", key, ttl] => {
                let ttl = ttl.parse::<u64>();
                match ttl {
                    Ok(ttl_from_input) => Command::Expire {
                        key: key.to_string(),
                        ttl: ttl_from_input,
                    },
                    Err(_) => Command::Unknown(format!("Invalid TTL: {:?}", ttl)),
                }
            }
            ["TTL", key] => Command::TTL {
                key: key.to_string(),
            },
            ["KEYS"] => Command::Keys,
            ["COUNT"] => Command::Count,
            ["CLEAR"] => Command::Clear,
            ["EXIT"] => Command::Exit,
            _ => Command::Unknown(format!("Unknown command {}", input.to_string())),
        }
    }
}
