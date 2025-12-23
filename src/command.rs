pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Rename { old_key: String, new_key: String },
    Keys,
    Count,
    Clear,
    Exit,
    Unknown (String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            ["GET", key] => Command::Get { key: key.to_string() },
            ["SET", key, value] => Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            },
            ["DELETE", key] => Command::Delete { key: key.to_string() },
            ["RENAME", old_key, new_key] => Command::Rename {
                old_key: old_key.to_string(),
                new_key: new_key.to_string(),
            },
            ["KEYS"] => Command::Keys,
            ["COUNT"] => Command::Count,
            ["CLEAR"] => Command::Clear,
            ["EXIT"] => Command::Exit,
            _ => Command::Unknown(input.to_string()),
        }
    }
}