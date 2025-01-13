mod blockchain;
use blockchain::Blockchain;

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

    // compute hashes for each block
    for block in blockchain.iter() {
        let hash_value = block.compute_hash();
        println!("Hash value of block: content: {}, hash: {} is {}", 
            block.content, block.hash, hash_value);
    }
    
    if let Some(removed) = blockchain.pop() {
        println!("Removed element: {}, {}", removed.content, removed.hash);
    }

    println!("The blockchain contains after removeing last node:");
    blockchain.display();
    
}

