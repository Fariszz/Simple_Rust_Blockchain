use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    hash: String,
    prev_hash: String,
    data: String,
    timestamp: u128
}

impl Block {
    fn new(prev_hash: String, data: String) -> Self {
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

fn main() {

    let block_one = Block::new("".to_string(), "hello world".to_string());

    println!("{:?}", block_one);

    let block_two = Block::new(block_one.hash, "hello world 2".to_string());

    println!("{:?}", block_two);

    let block_three = Block::new(block_two.hash, "hello world 3".to_string());

    println!("{:?}", block_three);
}


// create hash data from block
fn calculate_hash(previous_hash: String, data: String, timestamp: u128) -> String {
    let input = format!("{}{}{}", previous_hash, data, timestamp);
    digest(&input)
}