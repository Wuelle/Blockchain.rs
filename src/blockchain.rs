use crate::transaction::SignedTransaction;

pub struct Block<'a>{
    transactions: Vec<SignedTransaction<'a>>, // This should be a merkle tree 
    merkle_root_hash: Vec<u8>,
    nonce: i32,
}

pub struct Blockchain<'a>{
    blocks: Vec<Block<'a>>,
}

impl<'a> Blockchain<'a>{
    //pub fn new() -> Self{
    //    // Create the genesis block
    //    gen = Block{

    //    }
    //}
}
