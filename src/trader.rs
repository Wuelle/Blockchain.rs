use log::{info, trace, warn};
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::rngs::OsRng;
use crate::blockchain::{Block, Blockchain};
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{get_unix_timestamp, sha256_digest};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct Trader{
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    known_miners: Vec<Sender<Transaction>>, // Contains channels to every other miner
    is_miner: bool,
    blockchain: Blockchain,
}

impl Trader{
    pub fn new<F>(f: F, is_miner: bool, miners: &mut Vec<Sender<SignedTransaction>>, traders: &mut Vec<Sender<Block>>) -> Self 
    where F: FnOnce() {
        let mut rng = OsRng;
        let private_key = RSAPrivateKey::new(&mut rng, 2048)
            .expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        if is_miner{
            let (sender, receiver): (Sender<SignedTransaction>, Receiver<SignedTransaction>) = mpsc::channel();
            let handle = Trader::spawn_miner_thread(receiver, Vec::new());
            miners.push(sender);
        }

        Trader{
            public_key: public_key,
            private_key: private_key,
            known_miners: Vec::new(),
            is_miner: is_miner,
            blockchain: Blockchain::new(),
        }
    }

    /// Sign a given Transaction with the RSA private key
    pub fn sign(&self, t: Transaction) -> SignedTransaction {
        let hashed = sha256_digest(&t);
        println!("{:?} is the initial hash", hashed);
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = self.private_key.sign(padding, &hashed).unwrap();
        
        println!("{:?} is signature old", s);
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }
    
    /// Spawn a new thread that listens for incoming transactions and keeps track of the local blockchain
    pub fn spawn_trader_thread(rb: Receiver<Block>) -> JoinHandle<()> {
        info!("Starting a new Trader thread!");
        thread::spawn(move|| {
            loop {
                let b = rb.recv().unwrap();
                trace!("Trader thread just received a new transaction!");
            }
        })
    }

    /// Start a new miner thread listening for incoming transactions
    pub fn spawn_miner_thread(rt: Receiver<SignedTransaction>, sb: Vec<Sender<Block>>) -> JoinHandle<()>{
        // The mining policy can vary from miner to miner, this is a rather simple one:
        // the miner waits for a fixed number of transactions to arrive before he 
        // starts mining a new block, regardless of tips etc.
        info!("Spawning a new miner thread!");
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
                if digest[0] == 0{
                    info!("Found matching nonce {:?}, results in {:?}", nonce, digest);
                    nonce_found = true;
                }
                else{
                    info!("DOESNT MATCH {:?}", nonce);
                }
                nonce += 1;
            }
            for peer in sb{
                peer.send(b.clone()).unwrap();
            }
        })
    }
}
