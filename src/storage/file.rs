use std::collections::HashMap;
use std::fs;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use crate::storage::Item;

const HEADER_MARKER: u8 = 0xFB;
const EXPIRY_MILLISECONDS: u8 = 0xFC;
const EXPIRY_SECONDS: u8 = 0xFD;

// Ref: https://rdb.fnordig.de/file_format.html
pub fn read_file(config: &HashMap<String, String>) -> HashMap<String, Item> {
    let mut storage = HashMap::new();

    let contents = match get_content(config) {
        Some(contents) => contents,
        None => return storage,
    };

    let mut iter = contents.into_iter().skip_while(|&b| b != HEADER_MARKER).skip(1);
    let hashtable_size = iter.next().unwrap_or(0) as usize;
    let expire_hashtable_size = iter.next().unwrap_or(0) as usize;
    println!("Hashtable Size: {}, Expire Size: {}", hashtable_size, expire_hashtable_size);

    for _ in 0..hashtable_size {
        let (_, expiry) = extract_expiry(&mut iter);
        let key = extract_string(&mut iter);
        let value = extract_string(&mut iter);

        println!("Loaded from file = key: {:?}, value: {:?}", key, value);

        let in_ms = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        if expiry > 0 && expiry < in_ms {
            continue;
        }

        storage.insert(
            key,
            Item {
                value,
                created: Instant::now(),
                expires_ms: expiry - in_ms,
            },
        );
    }

    storage
}

fn get_content(config: &HashMap<String, String>) -> Option<Vec<u8>> {
    config
        .get("dir")
        .and_then(|dir| config.get("dbfilename").map(|filename| format!("{}/{}", dir, filename)))
        .and_then(|path| fs::read(path).ok())
}

fn extract_expiry(iter: &mut impl Iterator<Item = u8>) -> (u8, u128) {
    let mut value_type = iter.next().unwrap_or(0);
    let expiry: u128;
    match value_type {
        EXPIRY_MILLISECONDS => {
            expiry = extract_bytes(iter, 8).map_or(0, |bytes| u64::from_le_bytes(bytes) as u128);
            value_type = iter.next().unwrap();
        }
        EXPIRY_SECONDS => {
            expiry = extract_bytes(iter, 4).map_or(0, |bytes| (u32::from_le_bytes(bytes) * 1000) as u128);
            value_type = iter.next().unwrap();
        },
        _ => expiry = 0,
    };
    println!("KV type: {}, expiry: {:?}", value_type, expiry);
    (value_type, expiry)
}

fn extract_bytes<const N: usize>(iter: &mut impl Iterator<Item = u8>, size: usize) -> Option<[u8; N]> {
    let mut buffer = [0; N];
    for i in 0..size {
        buffer[i] = iter.next()?;
    }
    Some(buffer)
}

fn extract_string(iter: &mut impl Iterator<Item = u8>) -> String {
    let len = iter.next().unwrap_or(0);
    iter.take(len as usize).map(|b| b as char).collect()
}

