use rsa::{PaddingScheme, Hash as HashTypes, PublicKey};
use super::utils::{sha256_digest, any_as_u8_slice};
use rsa::RSAPublicKey;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Transaction{
    pub sender: RSAPublicKey,
    pub receiver: RSAPublicKey,
    pub amount: f32,
    pub change: f32,
    pub fee: f32,
}

#[derive(Clone, Debug)]
pub struct SignedTransaction{
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}

impl Hash for Transaction{
    /// TODO: This is bad, since unsafe
    fn hash<H: Hasher>(&self, state: &mut H){
        let bytes = unsafe{ any_as_u8_slice(&self.amount) };
        bytes.hash(state);
    }
}

impl Hash for SignedTransaction{
    fn hash<H: Hasher>(&self, state: &mut H) {
            self.signature.hash(state);
    }
}

impl Transaction{
    pub fn new(s: RSAPublicKey, r: RSAPublicKey, amount: f32) -> Transaction{
        Transaction{
            sender: s,
            receiver: r,
            amount: amount,
            change: 0.0,
            fee: 0.1, // TODO: allow adding a tip to the miner here 
        }
    }
}

impl SignedTransaction{
    pub fn is_valid(&self) -> bool{
        let hashed = sha256_digest(&self.transaction);
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(HashTypes::SHA2_256));
        self.transaction.sender.verify(padding, &hashed, &self.signature).is_ok()
    }
}

#[cfg(test)]
mod test{
    // Imports
    use super::*;
    use crate::trader::Trader;

    #[test]
    fn validate_signature(){
        let mut miners = Vec::new();
        let mut traders = Vec::new();

        let trader_1 = Trader::new(||{}, false, &mut miners, &mut traders);
        let trader_2 = Trader::new(||{}, false, &mut miners, &mut traders);

        // Assert that correct transactions are valid
        let t = Transaction::new(trader_1.public_key.clone(), trader_2.public_key.clone(), 1.0);
        let st_good = trader_1.sign(t);
        assert!(st_good.is_valid());

        // Assert that incorrect transactions are invalid
        let t_ = Transaction::new(trader_1.public_key.clone(), trader_2.public_key.clone(), 1.0);
        let st_bad = trader_2.sign(t_);
        assert!(!st_bad.is_valid());
    }
}
