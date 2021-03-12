use crate::transaction::SignedTransaction;
use crate::merkletree::MerkleTree;
use crate::utils::get_unix_timestamp;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Block{
    pub transactions: MerkleTree<SignedTransaction>, // This should be a merkle tree 
    pub nonce: i32,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>,
}

impl Hash for Block{
    fn hash<H: Hasher>(&self, state: &mut H) {
            self.transactions.get_root_hash().hash(state);
            self.nonce.hash(state);
            self.timestamp.hash(state);
    }
}

impl Blockchain{
    pub fn new() -> Self{
        // Create the genesis block
        let gen = Block{
            transactions: MerkleTree::new(),
            nonce: 0,
            timestamp: get_unix_timestamp(),
        };
        Blockchain{
            blocks: vec![gen],
        }
    }

    pub fn add(&mut self, b: Block) {
        self.blocks.push(b);
    }
}

impl Block {
    pub fn is_valid(&self) -> bool {
        self.transactions.root.is_valid()
    }
}
