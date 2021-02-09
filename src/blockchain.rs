use crate::transaction::SignedTransaction;

pub struct Block{
    transactions: Vec<SignedTransaction>, // This should be a merkle tree 
    merkle_root_hash: Vec<u8>,
    nonce: i32,
}

pub struct Blockchain{
    blocks: Vec<Block>,
}

impl Blockchain{
    //pub fn new() -> Self{
    //    // Create the genesis block
    //    gen = Block{

    //    }
    //}
}
