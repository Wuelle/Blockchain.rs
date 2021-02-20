use rsa::{PaddingScheme, Hash as HashTypes, PublicKey};
use super::utils::{sha256_digest, any_as_u8_slice};
use rsa::RSAPublicKey;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Transaction{
    pub sender: RSAPublicKey,
    pub receiver: RSAPublicKey,
    pub amount: f32,
    pub change: f32,
    pub fee: f32,
}

#[derive(Clone)]
pub struct SignedTransaction{
    pub transaction: Transaction,
    pub signature: Vec<u8>,
}

impl Hash for Transaction{
    /// TODO: This is bad, since unsafe
    fn hash<H: Hasher>(&self, state: &mut H){
        let bytes = unsafe{ any_as_u8_slice(&self) };
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
        let hashed = sha256_digest(&self.transaction).to_ne_bytes();
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(HashTypes::SHA2_256));
        self.transaction.sender.verify(padding, &hashed, &self.signature).is_ok()
    }
}

