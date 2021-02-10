use crate::transaction::SignedTransaction;
use crate::utils::get_unix_timestamp;
use std::sync::Arc;

pub struct Block{
    pub transactions: Vec<Arc<SignedTransaction>>, // This should be a merkle tree 
    pub merkle_root_hash: Vec<u8>,
    pub nonce: i32,
    pub timestamp: u64,
    pub version: String,
}

pub struct Blockchain{
    blocks: Vec<Block>,
}


impl Blockchain{
    pub fn new() -> Self{
        // Create the genesis block
        let gen = Block{
            transactions: Vec::new(),
            merkle_root_hash: Vec::new(),
            nonce: 0,
            timestamp: get_unix_timestamp(),
            version: "0.21.0".to_string(),
        };
        Blockchain{
            blocks: vec![gen],
        }
    }
}
