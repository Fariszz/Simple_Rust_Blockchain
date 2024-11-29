use sha256::digest;
use std::collections::LinkedList;
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

    let mut blockchain: LinkedList<Block> = LinkedList::new();

    blockchain.push_back(Block::new("".to_string(), "hello world".to_string()));

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 2".to_string()));

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 3".to_string()));

    for block in &blockchain {
        println!("{:?}", block);
    }
}


// create hash data from block
fn calculate_hash(previous_hash: String, data: String, timestamp: u128) -> String {
    let input = format!("{}{}{}", previous_hash, data, timestamp);
    digest(&input)
}

fn is_valid_block(block: &Block, previous_block: &Block) -> bool {
    block.prev_hash == previous_block.hash && block.hash == calculate_hash(previous_block.hash.clone(), block.data.clone(), block.timestamp)
}