use log::{info, warn};
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::rngs::OsRng;
use crate::blockchain::{Block, Blockchain};
use crate::merkletree::MerkleTree;
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{random_id, sha256_digest, get_unix_timestamp};
use std::{
    thread,
    thread::JoinHandle,
    sync::{
        mpsc,
        Arc,
        Mutex,
        mpsc::{Receiver, Sender}
    }
};

type STSender = Sender<SignedTransaction>;
type STReceiver = Receiver<SignedTransaction>;
type Shared<T> = Arc<Mutex<T>>;

pub struct Trader {
    id: String,
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    pub known_miners: Shared<Vec<STSender>>,
    known_traders: Shared<Vec<Sender<Block>>>,
    blockchain: Shared<Blockchain>,
    block_sender: Sender<Block>,
}

impl Trader{
    pub fn new() -> Trader {
        // Generate a random 256bit RSA key pair
        let mut rng = OsRng;
        let private_key = RSAPrivateKey::new(&mut rng, 512).expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        let (block_sender, block_receiver) = mpsc::channel();
        let blockchain = Arc::new(Mutex::new(Blockchain::new()));
        let id = random_id(5);

        Trader::spawn_trader_thread(&id, blockchain.clone(), block_receiver);

        Trader {
            id: id, 
            public_key: public_key.clone(),
            private_key: private_key,
            blockchain: blockchain.clone(),
            known_miners: Arc::new(Mutex::new(Vec::new())),
            known_traders: Arc::new(Mutex::new(Vec::new())),
            block_sender: block_sender,
        }
    }
    
    /// Spawn a new thread that listens for incoming transactions and keeps track of the local blockchain
    pub fn spawn_trader_thread(id: &str, blockchain: Arc<Mutex<Blockchain>>, block_receiver: Receiver<Block>) -> JoinHandle<()> {
        info!("Spawning new Trader thread {}", id);
        let name = format!("[Trader]{}", id);
        thread::Builder::new().name(name).spawn(move|| {
            loop {
                // Check for new transactions to be added to the blockchain
                match block_receiver.try_recv() {
                    Ok(block) => {
                        if block.is_valid() {
                            info!("Received new Block, now adding it to the Blockchain");
                            // Acquire thread lock
                            if let Ok(mut bc) = blockchain.lock() {
                                bc.add(block);
                            }
                        }
                        else {
                            warn!("Received an invalid block");
                        }
                    },
                    Err(error) => {
                        if let mpsc::TryRecvError::Disconnected = error {
                            panic!("Traders transaction channel disconnected");
                        }
                    },
                };

            }
        }).unwrap()
    }

    pub fn spawn_miner_thread(&self) -> Sender<SignedTransaction>{
        // Clone Mutexes
        let blockchain = self.blockchain.clone();
        let known_traders = self.known_traders.clone();
        let (transaction_sender, transaction_receiver): (STSender, STReceiver) = mpsc::channel();

        // The mining policy can vary from miner to miner, this is a rather simple one:
        // the miner waits for a fixed number of transactions to arrive before he 
        // starts mining a new block, regardless of tips etc.
        info!("Spawning new Miner thread {}", self.id);
        let name = format!("[Miner]{}", self.id);
        thread::Builder::new().name(name).spawn(move|| {
            loop {
                let mut previous_hash = Vec::new();
                if let Ok(bc) = blockchain.lock() {
                    previous_hash = sha256_digest(&bc.blocks[bc.blocks.len() - 1]);
                }

                let mut b = Block {
                    id: random_id(10),
                    transactions: MerkleTree::new(),
                    nonce: 0,
                    timestamp: get_unix_timestamp(),
                    previous_hash: previous_hash, 
                };

                // Wait for a single transaction
                info!("Waiting for new transactions");
                while b.transactions.len() < 1 {
                    let t = transaction_receiver.recv().unwrap();

                    // Validate the Transaction before adding it to the block
                    if t.is_valid(){
                        info!("Received a new, valid transaction");
                        b.transactions.add(t);
                    }
                    else{
                        warn!("Received an invalid transaction");
                    }
                }

                // Find Proof-of-Work
                let mut nonce_found = false;
                let mut nonce = 0;
                info!("Starting to search for the correct nonce");
                while !nonce_found {
                    b.nonce = nonce;
                    
                    let digest = sha256_digest(&b);
                    if digest[0] == 0{
                        info!("Solved: {:?}", nonce);
                        nonce_found = true;
                    }
                    nonce += 1;
                }

                // Send the solved block to all other traders
                if let Ok(mut traders) = known_traders.lock() {
                    for peer in traders.iter_mut() {
                        peer.send(b.clone()).unwrap();
                    }
                }
            }
        }).unwrap();
        transaction_sender
    }

    /// Sign a given Transaction with the RSA private key
    pub fn sign(&self, t: Transaction) -> SignedTransaction {
        let hashed = sha256_digest(&t);
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA2_256));
        let s = self.private_key.sign(padding, &hashed).unwrap();
        
        SignedTransaction{
            transaction: t,
            signature: s
        }
    }

    /// Link to two traders together, creating a p2p network
    pub fn link(&self, partner: &mut Trader){
        if let Ok(mut traders) = self.known_traders.lock() {
            traders.push(partner.block_sender.clone());
        }
        if let Ok(mut traders) = partner.known_traders.lock() {
            traders.push(self.block_sender.clone());
        }
    }

    pub fn register_miner(&self, miner: &Sender<SignedTransaction>) {
        if let Ok(mut miners) = self.known_miners.lock() {
            miners.push(miner.clone());
        }
    }

    /// Broadcast transaction to miners
    pub fn broadcast(&self, transaction: &SignedTransaction) {
        if let Ok(mut miners) = self.known_miners.lock() {
            for miner in miners.iter_mut() {
                miner.send(transaction.clone()).unwrap();
            }
        }
    }
}

