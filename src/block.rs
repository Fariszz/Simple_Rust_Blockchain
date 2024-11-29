use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub prev_hash: String,
    pub data: String,
    pub timestamp: u128
}

impl Block {
    pub fn new(prev_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

        Self {
            hash: calculate_hash(prev_hash.clone(), data.clone(), timestamp),
            prev_hash,
            data,
            timestamp
        }
    }
}

// create hash data from block
pub fn calculate_hash(previous_hash: String, data: String, timestamp: u128) -> String {
    let input = format!("{}{}{}", previous_hash, data, timestamp);
    digest(&input)
}
