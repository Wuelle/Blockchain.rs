use rsa::{PaddingScheme, Hash, PublicKey};
use sha2::{Digest, Sha256};

use super::trader::Trader;
use super::utils::{any_as_u8_slice};

pub struct Transaction<'a>{
    sender: &'a Trader,
    receiver: &'a Trader,
    amount: f32,
    fee: f32,
}

pub struct SignedTransaction<'a>{
    pub transaction: &'a Transaction<'a>,
    pub signature: Vec<u8>,
}

impl<'a> SignedTransaction<'a>{
    pub fn verify(&self) -> bool{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&self.transaction) };
        let hashed = Sha256::digest(&bytes).to_vec();

        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
    
        self.transaction.sender.public_key.verify(padding, &hashed, &self.signature).is_ok()
    }
}

impl<'a> Transaction<'a>{
    pub fn new(s: &'a Trader, r: &'a Trader, amount: f32) -> Transaction<'a>{
        Transaction{
            sender: s,
            receiver: r,
            amount: amount,
            fee: 0.1, // TODO: allow adding a tip to the miner here 
        }
    }
}
