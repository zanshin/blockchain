use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// Block struct - each node is a block
#[derive(Hash)]
pub struct Block {
    pub content: String,
    pub hash: u64,
}

impl Block {
    // compute a simple hash value for a block
    pub fn compute_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

// Blockchain - an array of Block structs
#[derive(Hash)]
pub struct Blockchain {
    pub items: Vec<Block>,
}

impl Blockchain {

    // create a new, empty Blockchain
    pub fn new() -> Self {
        Self { items: Vec::new()  }
    }

    // add a new block to the chain
    pub fn push(&mut self, content: String, hash: u64) {
        self.items.push(Block { content, hash });
    }

    // remove the last block from the chain
    pub fn pop(&mut self) -> Option<Block> {
        self.items.pop()
    }

    // display all blocks in the blockchain
    pub fn display(&self) {
        for node in &self.items {
            println!("Content: {}, Hash: {}", node.content, node.hash);
        }
    }

    // create an iterator for the blockchain
    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.items.iter()
    }
}
