use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::core::error::{FerrisError, Result};
use crate::core::store::StoreData;
use super::Persistence;

pub struct FilePersistence {
    path: PathBuf,
}

impl FilePersistence {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl Persistence for FilePersistence {
    fn save(&self, data: &HashMap<String, StoreData>) -> Result<()> {
        let json = serde_json::to_string(data)
            .map_err(|e| FerrisError::Persistence(e.to_string()))?;
        fs::write(&self.path, json)
            .map_err(|e| FerrisError::Persistence(e.to_string()))
    }

    fn load(&self) -> Result<HashMap<String, StoreData>> {
        let contents = fs::read_to_string(&self.path)
            .map_err(|e| FerrisError::Persistence(e.to_string()))?;
        serde_json::from_str(&contents)
            .map_err(|e| FerrisError::Persistence(e.to_string()))
    }
}