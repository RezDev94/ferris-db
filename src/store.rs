use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use std::{fs, time::Duration};

const DATA_FILE: &str = "data.json";

#[derive(Serialize, Deserialize)]
struct StoreData {
    value: String,
    #[serde(with = "system_time_serde")]
    ttl: Option<SystemTime>,
}

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

#[derive(Serialize, Deserialize)]
pub struct Store {
    data: HashMap<String, StoreData>,
}

impl Store {
    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn new() -> Self {
        let store = Store { data: HashMap::new() };
        match Store::load() {
            Ok(store) => store,
            Err(_) => store,
        }
    }

    pub fn set(&mut self, key: String, value: String, ttl: Option<u64>) -> Result<(), String> {
        if self.exists(&key) {
            return Err("Key already exists".to_string());
        }
        self.data.insert(key, StoreData {
            value,
            ttl: ttl.map(|t| SystemTime::now() + Duration::from_secs(t)),
        });
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data
            .get(key)
            .map(|data| &data.value)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        if !self.exists(key) {
            return Err("Key does not exist".to_string());
        }
        self.data.remove(key);
        self.save()
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
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

        let data = self.data.remove(&old_key).unwrap();

        self.data.insert(new_key, data);
        self.save()
    }

    pub fn expire(&mut self, key: String, ttl: u64) -> Result<(), String> {
        if !self.exists(&key) {
            return Err("Key does not exist".to_string());
        }

        let element = self.data.get_mut(&key).unwrap();
        element.ttl = Some(SystemTime::now() + Duration::from_secs(ttl));
        self.save()
    }

    pub fn ttl(&self, key: String) -> Result<Option<u64>, String> {
        if !self.exists(&key) {
            return Err("Key does not exist".to_string());
        }

        let data = self.data.get(&key).unwrap();
        match data.ttl {
            None => Ok(None),
            Some(expiry) => {
                let duration = expiry.duration_since(SystemTime::now()).unwrap();
                Ok(Some(duration.as_secs()))
            }
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
