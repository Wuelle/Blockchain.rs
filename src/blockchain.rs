use crate::transaction::SignedTransaction;
use crate::merkletree::MerkleTree;
use crate::utils::{get_unix_timestamp, sha256_digest};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash)]
pub struct Block{
    pub id: String,
    pub transactions: MerkleTree<SignedTransaction>, 
    pub nonce: i32,
    pub timestamp: u64,
    pub previous_hash: Vec<u8>,
}

#[derive(Debug)]
pub struct Blockchain{
    pub blocks: Vec<Block>,
}

//impl Hash for Block{
//    fn hash<H: Hasher>(&self, state: &mut H) {
//            self.id.hash(state);
//            self.nonce.hash(state);
//            self.timestamp.hash(state);
//            self.previous_hash.hash(state);
//            self.transactions.get_root_hash().hash(state);
//    }
//}

impl Blockchain{
    pub fn new() -> Self{
        // Create the genesis block
        let gen = Block{
            id: "Genesis".to_string(),
            transactions: MerkleTree::new(),
            nonce: 0,
            timestamp: get_unix_timestamp(),
            previous_hash: Vec::new(),
        };
        Blockchain{
            blocks: vec![gen],
        }
    }

    pub fn add(&mut self, b: Block) {
        self.blocks.push(b);
    }

    pub fn is_valid(&self) -> bool {
        let mut is_valid = true;
        // By definition, the genesis block cannot be invalid
        for ix in 1..self.blocks.len() {
            let block = &self.blocks[ix];
            if !block.is_valid() {
                is_valid = false;
                break;
            }
            //Check if this blocks prev hash matches the previous blocks hash!
            let prev_block = &self.blocks[ix - 1];
            if !sha256_digest(&prev_block)
                .iter()
                .zip(&block.previous_hash)
                .all(|(a, b)| a == b) {
                is_valid = false;
                break
            }
        }
        is_valid
    }

}

impl Block {
    pub fn is_valid(&self) -> bool {
        self.transactions.root.is_valid()
    }
}
