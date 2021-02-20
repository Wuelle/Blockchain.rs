use crate::transaction::SignedTransaction;
use crate::utils::get_unix_timestamp;
use rsa::RSAPublicKey;

#[derive(Hash)]
pub struct Block{
    pub transactions: Vec<SignedTransaction>, // This should be a merkle tree 
    pub merkle_root_hash: Vec<u8>,
    pub nonce: i32,
    pub timestamp: u64,
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
        };
        Blockchain{
            blocks: vec![gen],
        }
    }
    
    fn get_balance(&self, adress: RSAPublicKey) -> f32{
        let mut total = 0.0;

        for block in &self.blocks {
            for t in &block.transactions {
                if t.transaction.sender == adress{
                    total -= t.transaction.amount;
                    total += t.transaction.change;
                    total -= t.transaction.fee;
                }
                else if t.transaction.receiver == adress{
                    total += t.transaction.amount;
                }
            }
        }
        total

    }
}
