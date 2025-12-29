use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::{fs, time::Duration};

const DATA_FILE: &str = "data.json";

#[derive(Serialize, Deserialize)]
struct StoreData {
    key: String,
    value: String,
    #[serde(with = "system_time_serde")]
    ttl: SystemTime,
}

mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).unwrap();
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_secs(secs))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Store {
    data: Vec<StoreData>,
}

impl Store {
    fn exists(&self, key: &str) -> bool {
        self.data.iter().any(|data| data.key == key)
    }

    pub fn new() -> Self {
        let store = Store { data: vec![] };
        match Store::load() {
            Ok(store) => store,
            Err(_) => store,
        }
    }

    pub fn set(&mut self, key: String, value: String, ttl: u64) -> Result<(), String> {
        if self.exists(&key) {
            return Err("Key already exists".to_string());
        }
        self.data.push(StoreData {
            key,
            value,
            ttl: SystemTime::now() + Duration::from_secs(ttl),
        });
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let found_element = self
            .data
            .iter()
            .find(|data| data.key == key && data.ttl > SystemTime::now());
        match found_element {
            Some(element) => Some(&element.value),
            None => None,
        }
    }

    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        if !self.exists(key) {
            return Err("Key does not exist".to_string());
        }
        self.data.retain(|data| data.key != key);
        self.save()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.iter().map(|data| &data.key).collect()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) -> Result<(), String> {
        self.data.clear();
        self.save()
    }

    pub fn rename(&mut self, old_key: String, new_key: String) -> Result<(), String> {
        if !self.exists(&old_key) {
            return Err("Key does not exist".to_string());
        }

        if self.exists(&new_key) {
            return Err("New key already exists".to_string());
        }

        let index = self
            .data
            .iter()
            .position(|data| data.key == old_key)
            .unwrap();
        self.data[index].key = new_key;
        self.save()
    }

    pub fn expire(&mut self, key: String, ttl: u64) -> Result<(), String> {
        if !self.exists(&key) {
            return Err("Key does not exist".to_string());
        }

        let element = self.data.iter_mut().find(|data| data.key == key).unwrap();
        element.ttl = SystemTime::now() + Duration::from_secs(ttl);
        self.save()
    }

    pub fn ttl(&self, key: String) -> Result<u64, String> {
        if !self.exists(&key) {
            return Err("Key does not exist".to_string());
        }

        let element = self.data.iter().find(|data| data.key == key).unwrap();
        let ttl = element
            .ttl
            .duration_since(SystemTime::now());
        match ttl {
            Ok(duration) => Ok(duration.as_secs()),
            Err(_) => Err("Key is expired".to_string()),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string(&self.data).map_err(|e| e.to_string())?;
        fs::write(DATA_FILE, json).map_err(|e| e.to_string())
    }

    pub fn load() -> Result<Store, String> {
        let contents = fs::read_to_string(DATA_FILE).map_err(|e| e.to_string())?;
        let data = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
        Ok(Store { data })
    }
}
