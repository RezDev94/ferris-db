use std::{collections::HashMap, fs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn new() -> Self {
        let store = Store { data: HashMap::new() };
        match Store::load() {
            Ok(store) => store,
            Err(_) => {
                store
            }
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        if self.exists(&key) {
            return Err("Key already exists".to_string());
        }
        self.data.insert(key, value);
        self.save()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
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

        let value = self.data.remove(&old_key).unwrap();
        self.data.insert(new_key, value);
        self.save()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = "data.json";
        let json = serde_json::to_string(&self.data)
            .map_err(|e| e.to_string())?;
        fs::write(path, json)
            .map_err(|e| e.to_string())
    }

    pub fn load() -> Result<Store, String> {
        let path = "data.json";
        let contents = fs::read_to_string(path)
            .map_err(|e| e.to_string())?;
        let data = serde_json::from_str(&contents)
            .map_err(|e| e.to_string())?;
        Ok(Store { data })
    }
}