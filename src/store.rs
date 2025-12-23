use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    pub fn new() -> Self {
        Store { data: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), String> {
        if self.exists(&key) {
            return Err("Key already exists".to_string());
        }
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        if !self.exists(key) {
            return Err("Key does not exist".to_string());
        }
        self.data.remove(key);
        Ok(())
    }

    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
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
        Ok(())
    }
}