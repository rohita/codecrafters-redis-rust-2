mod file;

use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Item {
    value: String,
    created: Instant,
    expires_ms: u128,
}

impl Item {
    pub fn is_expired(&self) -> bool {
        self.expires_ms > 0 && self.created.elapsed().as_millis() > self.expires_ms
    }
}

#[derive(Clone)]
pub struct Db {
    config: HashMap<String, String>,
    storage: HashMap<String, Item>,
}

impl Db {
    pub fn from_config(config: HashMap<String, String>) -> Self {
        let storage = file::read_file(&config);
        Db { config, storage }
    }

    pub fn set(&mut self, key: String, value: String, expires_ms: u128) {
        let item = Item {
            value,
            created: Instant::now(),
            expires_ms,
        };
        self.storage.insert(key, item);
    }

    pub fn get(&self, key: &String) -> Option<String> {
        let item = self.storage.get(key)?;
        match item.is_expired() {
            true => None,
            false => Some(item.value.clone()),
        }
    }

    pub fn config_get(&self, key: &String) -> Option<String> {
        self.config.get(key).cloned()
    }

    pub fn keys(&self) -> Vec<String> {
        self.storage.keys().cloned().collect()
    }
}
