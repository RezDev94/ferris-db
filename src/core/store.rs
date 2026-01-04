use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use super::error::{FerrisError, Result};
use crate::persistence::Persistence;

mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(t) => {
                let duration = t.duration_since(UNIX_EPOCH).unwrap();
                Some(duration.as_secs()).serialize(serializer)
            }
            None => None::<u64>.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs: Option<u64> = Option::deserialize(deserializer)?;
        Ok(secs.map(|s| UNIX_EPOCH + Duration::from_secs(s)))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StoreData {
    value: String,
    #[serde(with = "system_time_serde")]
    ttl: Option<SystemTime>,
}

pub struct Store {
    data: HashMap<String, StoreData>,
    persistence: Arc<dyn Persistence>,
}

impl Store {
    pub fn new(persistence: Arc<dyn Persistence>) -> Self {
        let data = persistence.load().unwrap_or_default();
        Store { data, persistence }
    }

    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn set(&mut self, key: String, value: String, ttl: Option<u64>) -> Result<()> {
        if self.exists(&key) {
            return Err(FerrisError::KeyExists(key));
        }
        self.data.insert(
            key,
            StoreData {
                value,
                ttl: ttl.map(|t| SystemTime::now() + Duration::from_secs(t)),
            },
        );
        self.persistence.save(&self.data)
    }

    pub fn get(&self, key: &str) -> Result<&String> {
        self.data
            .get(key)
            .map(|data| &data.value)
            .ok_or_else(|| FerrisError::KeyNotFound(key.to_string()))
    }

    pub fn delete(&mut self, key: &str) -> Result<()> {
        if !self.exists(key) {
            return Err(FerrisError::KeyNotFound(key.to_string()));
        }
        self.data.remove(key);
        self.persistence.save(&self.data)
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) -> Result<()> {
        self.data.clear();
        self.persistence.save(&self.data)
    }

    pub fn rename(&mut self, old_key: String, new_key: String) -> Result<()> {
        if !self.exists(&old_key) {
            return Err(FerrisError::KeyNotFound(old_key));
        }
        if self.exists(&new_key) {
            return Err(FerrisError::KeyExists(new_key));
        }
        let data = self.data.remove(&old_key).unwrap();
        self.data.insert(new_key, data);
        self.persistence.save(&self.data)
    }

    pub fn expire(&mut self, key: String, ttl: u64) -> Result<()> {
        let element = self
            .data
            .get_mut(&key)
            .ok_or_else(|| FerrisError::KeyNotFound(key))?;
        element.ttl = Some(SystemTime::now() + Duration::from_secs(ttl));
        self.persistence.save(&self.data)
    }

    pub fn ttl(&self, key: &str) -> Result<Option<u64>> {
        let data = self
            .data
            .get(key)
            .ok_or_else(|| FerrisError::KeyNotFound(key.to_string()))?;

        match data.ttl {
            None => Ok(None),
            Some(expiry) => {
                let duration = expiry
                    .duration_since(SystemTime::now())
                    .unwrap_or(Duration::ZERO);
                Ok(Some(duration.as_secs()))
            }
        }
    }
}
