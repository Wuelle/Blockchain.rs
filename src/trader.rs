use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{any_as_u8_slice};

pub struct Trader{
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    pub id: String,
}

impl std::hash::Hash for Trader{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H){
        self.id.hash(state);
    }
}

impl<'a> Trader{
    pub fn new() -> Self{
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RSAPrivateKey::new(&mut rng, bits)
            .expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric) 
            .take(16)
            .map(char::from)
            .collect();

        Trader{
            public_key: public_key,
            private_key: private_key,
            id: id,
        }
    }
    /// Sign a given Transaction with the RSA private key
    pub fn sign(&self, t: &'a Transaction<'a>) -> SignedTransaction<'a>{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&t) };
        let hashed = Sha256::digest(&bytes).to_vec(); 
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = self.private_key.sign(padding, &hashed).unwrap();
        
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }
}
