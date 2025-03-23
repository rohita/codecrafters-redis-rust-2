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

    pub fn get_rdb_file(&self) -> Vec<u8> {
        b"\x52\x45\x44\x49\x53\x30\x30\x31\x31\xfa\x09\x72\x65\x64\x69\x73\
          \x2d\x76\x65\x72\x05\x37\x2e\x32\x2e\x30\xfa\x0a\x72\x65\x64\x69\
          \x73\x2d\x62\x69\x74\x73\xc0\x40\xfa\x05\x63\x74\x69\x6d\x65\xc2\
          \x6d\x08\xbc\x65\xfa\x08\x75\x73\x65\x64\x2d\x6d\x65\x6d\xc2\xb0\
          \xc4\x10\x00\xfa\x08\x61\x6f\x66\x2d\x62\x61\x73\x65\xc0\x00\xff\
          \xf0\x6e\x3b\xfe\xc0\xff\x5a\xa2".to_vec()
    }
}
