use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::collections::LinkedList;

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

    pub fn mine_block(&mut self, difficulty: u128) {
        let target = "0".repeat(difficulty as usize); // Membuat string target dengan `difficulty` jumlah "0"
        while &self.hash[0..difficulty as usize] != target {
            self.timestamp += 1;
            self.hash = calculate_hash(self.prev_hash.clone(), self.data.clone(), self.timestamp);
        }
        println!("Block Mined!!! : {}", self.hash);
    }

    pub fn is_chain_valid(blockchain: &LinkedList<Block>, difficulty: u128) -> bool {
        let target = "0".repeat(difficulty as usize); // Membuat string target dengan `difficulty` jumlah "0"

          // Loop melalui blockchain untuk memeriksa hash:
          for i in 1..blockchain.len() {
            let current_block = blockchain.iter().nth(i).unwrap();
            let previous_block = blockchain.iter().nth(i - 1).unwrap();

            // Bandingkan hash terdaftar dan hash yang dihitung:
            if current_block.hash != calculate_hash(current_block.prev_hash.clone(), current_block.data.clone(), current_block.timestamp) {
                println!("Current Hashes not equal");
                return false;
            }

            // Bandingkan previous hash dan previous hash yang terdaftar
            if previous_block.hash != current_block.prev_hash {
                println!("Previous Hashes not equal");
                return false;
            }

            // Periksa apakah hash telah diselesaikan
            if &current_block.hash[0..difficulty as usize] != target {
                println!("This block hasn't been mined");
                return false;
            }
        }
        true
    }

    
}

// create hash data from block
fn calculate_hash(previous_hash: String, data: String, timestamp: u128) -> String {
    let input = format!("{}{}{}", previous_hash, data, timestamp);
    digest(&input)
}