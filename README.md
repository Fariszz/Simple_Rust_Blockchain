# Simple Blockchain

A simple blockchain implementation in Rust that demonstrates the basic concepts of blockchain technology. This project creates a chain of blocks where each block contains data and is cryptographically linked to the previous block using SHA-256 hashing.

## Features

- Block creation with timestamp
- SHA-256 hashing
- Cryptographic linking between blocks
- Simple and educational implementation

## Prerequisites

- Rust (edition 2021)
- Cargo (Rust's package manager)

## Dependencies

- `sha256` - Version 1.4.0 (For cryptographic hashing)
- `serde` - Version 1.0.188 (For serialization and deserialization)
- `serde_json` - Version 1.0.105 (For JSON serialization and deserialization)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Fariszz/simple_rust_blockchain.git
cd simple_rust_blockchain
```

2. Build the project:
```bash
cargo build
```

3. Run the project:
```bash
cargo run
```

## Project Structure

- `src/main.rs` - Contains the main implementation of the blockchain
- `Cargo.toml` - Project configuration and dependencies

## Implementation Details

The project implements a basic `Block` structure with the following properties:
- `hash`: The current block's hash
- `prev_hash`: The previous block's hash
- `data`: The data stored in the block
- `timestamp`: Block creation timestamp

Each block is cryptographically linked to the previous block through its hash, which is calculated using:
- The previous block's hash
- The current block's data
- The current timestamp

## Example Usage

The current implementation demonstrates creating a chain of three blocks with simple string data:

```rust
let mut blockchain: LinkedList<Block> = LinkedList::new();

blockchain.push_back(Block::new("".to_string(), "hello world".to_string()));

blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 2".to_string()));

blockchain.push_back(Block::new(blockchain.back().unwrap().hash.clone(), "hello world 3".to_string()));
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
