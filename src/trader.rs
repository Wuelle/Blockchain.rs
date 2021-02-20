use log::{info, trace, warn};
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::rngs::OsRng;
use crate::blockchain::Block;
use crossbeam::channel::{Receiver, Sender};
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{get_unix_timestamp, sha256_digest};
use std::thread::{self, JoinHandle};

#[derive(Clone)]
pub struct Trader{
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
}

impl Trader{
    pub fn new() -> Self {
        let mut rng = OsRng;
        let bits = 2048;
        let private_key = RSAPrivateKey::new(&mut rng, bits)
            .expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        Trader{
            public_key: public_key,
            private_key: private_key,
        }
    }

    /// Sign a given Transaction with the RSA private key
    pub fn sign(&self, t: Transaction) -> SignedTransaction {
        let hashed = sha256_digest(&t);
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = self.private_key.sign(padding, &hashed.to_ne_bytes()).unwrap();
        
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }

    /// Start a new miner thread listening for incoming transactions
    pub fn spawn_miner_thread(&self, rt: Receiver<SignedTransaction>, sb: Sender<Block>) -> JoinHandle<()>{
        // The mining policy can vary from miner to miner, this is a rather simple one:
        // the miner waits for a fixed number of transactions to arrive before he 
        // starts mining a new block, regardless of tips etc.
        thread::spawn(move|| {
            let mut b = Block {
                transactions: Vec::new(),
                merkle_root_hash: Vec::new(),
                nonce: 0,
                timestamp: get_unix_timestamp(),
            };

            // Mine for just a single transaction
            while b.transactions.len() < 1 {
                let t = rt.recv().unwrap();
                println!("Received transaction!");

                // Validate the Transaction before adding it to the block
                if t.is_valid(){
                    b.transactions.push(t.clone());
                    b.merkle_root_hash = t.signature.clone();
                }
                else{
                    warn!("Received an invalid transaction!");
                }
            }

            trace!("Starting to mine some GOLD!");
            let mut nonce_found = false;
            let mut nonce = 0;

            while !nonce_found {
                b.nonce = nonce;
                
                let digest = sha256_digest(&b);
                if digest.to_ne_bytes()[0] == 0{
                    info!("Found matching nonce {:?}, results in {:?}", nonce, digest);
                    nonce_found = true;
                }
                else{
                    info!("DOESNT MATCH {:?}", nonce);
                }
                nonce += 1;
            }
            sb.send(b).unwrap();
        })
    }
}
