mod block;

use std::collections::LinkedList;
use block::Block;

const DIFFICULTY: u128 = 5;

fn main() {
    let mut blockchain: LinkedList<Block> = LinkedList::new();

    blockchain.push_back(Block::new("0".to_string(), "hello world".to_string()));
    println!("Trying to mine block 1...");
    blockchain.back_mut().unwrap().mine_block(DIFFICULTY);

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 2".to_string()));
    println!("Trying to mine block 2...");
    blockchain.back_mut().unwrap().mine_block(DIFFICULTY);

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 3".to_string()));
    println!("Trying to mine block 3...");
    blockchain.back_mut().unwrap().mine_block(DIFFICULTY);

    if Block::is_chain_valid(&blockchain, DIFFICULTY) {
        println!("\nBlockchain is valid");
    } else {
        println!("\nBlockchain is not valid");
    }

    for block in &blockchain {
        let serialize = serde_json::to_string_pretty(&block).unwrap();

        println!("\n{}", serialize);
    }
}