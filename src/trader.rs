use log::{info, trace, warn};
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use sha2::{Digest, Sha256};
use crate::blockchain::Block;
use crossbeam::channel::{Receiver, Sender};
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{any_as_u8_slice, get_unix_timestamp};
use std::thread::{self, JoinHandle};
use std::sync::Arc;

pub struct Trader{
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    pub id: String,
}

impl Trader{
    pub fn new() -> Arc<Self>{
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

        Arc::new(Trader{
            public_key: public_key,
            private_key: private_key,
            id: id,
        })
    }

    /// Sign a given Transaction with the RSA private key
    pub fn sign(&self, t: Transaction) -> SignedTransaction{
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&t) };
        let hashed = Sha256::digest(&bytes).to_vec(); 
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = self.private_key.sign(padding, &hashed).unwrap();
        
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }

    /// Start a new miner thread listening for incoming transactions
    pub fn spawn_miner_thread(&self, rt: Receiver<Arc<SignedTransaction>>, sb: Sender<Arc<Block>>) -> JoinHandle<()>{
        thread::spawn(move|| {
            println!("Child thread running!");

            // Mine for just a single transaction
            for _ in 0..1 {
                let t = rt.recv().unwrap();
                trace!("Starting to mine some GOLD!");
                let mut nonce_found = false;
                let mut nonce = 0;

                let mut b = Block {
                    transactions: vec![t.clone()],
                    merkle_root_hash: t.clone().signature.clone(),
                    nonce: 0,
                    timestamp: get_unix_timestamp(),
                    version: "0.21.0".to_string(),
                };
                while !nonce_found {
                    b.nonce = nonce;
                    
                    let bytes: &[u8] = unsafe{ any_as_u8_slice(&b) };
                    let digest = Sha256::digest(&bytes).to_vec();

                    if digest[0] == 0{
                        info!("Found matching nonce {:?}, results in {:?}", nonce, digest);
                        nonce_found = true;
                    }
                    else{
                        info!("DOESNT MATCH {:?}", nonce);
                    }
                    nonce += 1;
                }
                sb.send(Arc::new(b)).unwrap();
            }
        })
    }
}
