fn main() {
    println!("Hello Blockchain!");
    println!(" ");
    let block = Block::new();
    println!("value: {}, hash: {}", block.value, block.prev_block_hash);
}

// our blockchain is a struct that has value 
// and a hash of the previous blocl in the chain
struct Block {
    value: String,
    prev_block_hash: String,
}

impl Block {
    fn new() -> Self {
        Self {
            value: "[Genesis Block]".to_string(),
            prev_block_hash: "genesis".to_string(),
        }
    }
}

// make a simple hash of a block. this is very simple (read: insecure) hash
// function.
