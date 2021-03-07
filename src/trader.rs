use log::{info, trace, warn};
use rsa::PublicKeyParts;
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::rngs::OsRng;
use crate::blockchain::{Block, Blockchain};
use crate::merkletree::MerkleTree;
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::{get_unix_timestamp, sha256_digest};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};

// a closure to be sent to the trader thread, to enable the user to control trader behaviour from main()
type Command = Box<dyn FnOnce(&Trader) -> () + Send>;
type BlockSender = Sender<Block>;
type STSender = Sender<SignedTransaction>;
type STReceiver = Receiver<SignedTransaction>;

pub struct Trader {
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    known_miners: Vec<Sender<SignedTransaction>>, // Contains channels to every other miner
    blockchain: Blockchain,
}

pub struct TraderInterface {
    pub public_key: RSAPublicKey,
    command_sender: Sender<Command>,
    block_sender: Sender<Block>,
}

unsafe impl Send for Trader {}
unsafe impl Sync for Trader {}

impl Trader{
    pub fn new(is_miner: bool, miners: &mut Vec<STSender>, traders: &mut Vec<BlockSender>) -> TraderInterface {
        // Generate a random 256bit RSA key pair
        let mut rng = OsRng;
        let private_key = RSAPrivateKey::new(&mut rng, 512).expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

        if is_miner{
            let (sender, receiver): (STSender, STReceiver) = mpsc::channel();
            let handle = Trader::spawn_miner_thread(receiver, Vec::new());
            miners.push(sender);
        }
        let (command_sender, command_receiver) = mpsc::channel();
        let (block_sender, block_receiver) = mpsc::channel();

        let t = Trader {
            public_key: public_key.clone(),
            private_key: private_key,
            blockchain: Blockchain::new(),
            known_miners: Vec::new(),
        };
        t.spawn_trader_thread(block_receiver, command_receiver);
        TraderInterface {
            public_key: public_key,
            command_sender: command_sender,    
            block_sender: block_sender,
        }
    }
    
    /// Spawn a new thread that listens for incoming transactions and keeps track of the local blockchain
    pub fn spawn_trader_thread(mut self, block_receiver: Receiver<Block>, command_receiver: Receiver<Command>) -> JoinHandle<()> {
        info!("Starting a new Trader thread!");

        thread::spawn(move|| {
            loop {
                // Check for new transactions to be added to the blockchain
                match block_receiver.try_recv() {
                    Ok(block) => {
                        info!("The trader just received a new transaction!");
                        self.blockchain.add(block);
                    },
                    Err(error) => {
                        if let mpsc::TryRecvError::Disconnected = error {
                            panic!("Traders transaction channel disconnected");
                        }
                    },
                };

                // Check for commands from main() (like sending a new transaction)
                match command_receiver.try_recv() {
                    Ok(c) => {
                        info!("The trader just received a new command!");
                        c(&self);
                    },
                    Err(error) => {
                        if let mpsc::TryRecvError::Disconnected = error {
                            panic!("Traders command channel disconnected");
                        }
                    },
                };
            }
        })
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

    /// Start a new miner thread listening for incoming transactions
    pub fn spawn_miner_thread(rt: Receiver<SignedTransaction>, sb: Vec<Sender<Block>>) -> JoinHandle<()> {
        // The mining policy can vary from miner to miner, this is a rather simple one:
        // the miner waits for a fixed number of transactions to arrive before he 
        // starts mining a new block, regardless of tips etc.
        info!("Spawning a new miner thread!");
        thread::spawn(move|| {
            let mut b = Block {
                transactions: MerkleTree::new(),
                nonce: 0,
                timestamp: get_unix_timestamp(),
            };

            // Mine for just a single transaction
            while b.transactions.len() < 1 {
                let t = rt.recv().unwrap();
                println!("Received transaction!");

                // Validate the Transaction before adding it to the block
                if t.is_valid(){
                    b.transactions.add(t.clone());
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

impl TraderInterface {
    pub fn execute<F: 'static>(&self, command: F) where 
        F: FnOnce(&Trader) -> () + Send {
        self.command_sender.send(Box::new(command)).unwrap();
    }
}
