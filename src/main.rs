mod block;

use std::collections::LinkedList;
use block::Block;

fn main() {
    let mut blockchain: LinkedList<Block> = LinkedList::new();

    blockchain.push_back(Block::new("".to_string(), "hello world".to_string()));

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 2".to_string()));

    blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 3".to_string()));

    for block in &blockchain {
        let serialize = serde_json::to_string_pretty(&block).unwrap();

        println!("{}", serialize);
    }
}