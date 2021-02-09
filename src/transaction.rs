use rsa::{PaddingScheme, Hash, PublicKey};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use super::trader::Trader;
use super::utils::{any_as_u8_slice};

#[derive(Clone)]
pub struct Transaction{
    sender: Arc<Trader>,
    receiver: Arc<Trader>,
    amount: f32,
    fee: f32,
}

#[derive(Clone)]
pub struct SignedTransaction{
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}

impl Transaction{
    pub fn new(s: Arc<Trader>, r: Arc<Trader>, amount: f32) -> Transaction{
        Transaction{
            sender: s,
            receiver: r,
            amount: amount,
            fee: 0.1, // TODO: allow adding a tip to the miner here 
        }
    }
}

impl SignedTransaction{
    pub fn is_valid(&self) -> bool{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&self.transaction) };
        let hashed = Sha256::digest(&bytes).to_vec();

        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
    
        self.transaction.sender.public_key.verify(padding, &hashed, &self.signature).is_ok()
    }
}

