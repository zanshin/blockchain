fn main() {
    // create a new blockchain
    let mut blockchain = Blockchain::new();

    // add the initial block to the chain
    blockchain.push(String::from("Genesis"), 0);

    // add more blocks to the chain
    blockchain.push(String::from("Transaction 1"), 0);
    blockchain.push(String::from("Transaction 2"), 0);
    blockchain.push(String::from("Transaction 3"), 0);

    // display the contents of the chain
    println!("The blockchain contains:");
    blockchain.display();
    
    if let Some(removed) = blockchain.pop() {
        println!("Removed element: {}, {}", removed.content, removed.hash);
    }

    println!("The blockchain contains after removeing last node:");
    blockchain.display();
    
}

// Block struct - each node is a block
struct Block {
    content: String,
    hash: u64,
}

// Blockchain - an array of Block structs
struct Blockchain {
    items: Vec<Block>,
}

impl Blockchain {

    // create a new, empty Blockchain
    fn new() -> Self {
        Self { items: Vec::new()  }
    }

    // add a new block to the chain
    fn push(&mut self, content: String, hash: u64) {
        self.items.push(Block { content, hash });
    }

    // remove a block from the chair
    fn pop(&mut self) -> Option<Block> {
        self.items.pop()
    }

    // display all blocks in the blockchain
    fn display(&self) {
        for node in &self.items {
            println!("Content: {}, Hash: {}", node.content, node.hash);
        }
    }
}
