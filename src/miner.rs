use crate::blockchain::Block;
use crate::merkletree::MerkleTree;
use crate::transaction::SignedTransaction;
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use std::sync::mpsc::{self, Receiver, Sender};
use log::{info, warn};
use std::thread::{self, JoinHandle};
use crate::utils::{get_unix_timestamp, sha256_digest};

type Command = Box<dyn FnOnce(&Miner) -> () + Send>;
type STSender = Sender<SignedTransaction>;
type STReceiver = Receiver<SignedTransaction>;

pub struct MinerInterface {
    pub public_key: RSAPublicKey,
    pub transaction_sender: STSender,
}

pub struct Miner {
    public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    transaction_receiver: STReceiver,
    pub known_traders: Vec<Sender<Block>>,
}

impl Miner {
    ///// Create a new miner linked to the provided Bitcoin adress
    ///// A miner is essentially the same as a trader except it's local blockchain is stored inside an Arc<Mutex<>>
    ///// So both the mining thread and the trader thread can access the same data at the same time
    //pub fn new(trader: &TraderInterface) -> MinerInterface {
    //    let (transaction_sender, transaction_receiver): (STSender, STReceiver) = mpsc::channel();
    //    let m = Miner {
    //        public_key: trader.public_key.clone(),
    //        private_key: trader.private_key.clone(),
    //        transaction_receiver: transaction_receiver,
    //        known_traders: vec![trader.block_sender.clone()],
    //    };

    //    // The mining policy can vary from miner to miner, this is a rather simple one:
    //    // the miner waits for a fixed number of transactions to arrive before he 
    //    // starts mining a new block, regardless of tips etc.
    //    info!("Spawning a new miner thread!");
    //    thread::spawn(move|| {
    //        loop {
    //            info!("Creating a new Block");
    //            let mut b = Block {
    //                transactions: MerkleTree::new(),
    //                nonce: 0,
    //                timestamp: get_unix_timestamp(),
    //            };

    //            // Wait for a single transaction
    //            while b.transactions.len() < 1 {
    //                let t = m.transaction_receiver.recv().unwrap();

    //                // Validate the Transaction before adding it to the block
    //                if t.is_valid(){
    //                    info!("Received a new, valid transaction");
    //                    b.transactions.add(t.clone());
    //                }
    //                else{
    //                    warn!("Received an invalid transaction");
    //                }
    //            }

    //            // Find Proof-of-Work
    //            let mut nonce_found = false;
    //            let mut nonce = 0;
    //            info!("Starting to search for the correct nonce");
    //            while !nonce_found {
    //                b.nonce = nonce;
    //                
    //                let digest = sha256_digest(&b);
    //                if digest[0] == 0{
    //                    info!("Solved: {:?}", nonce);
    //                    nonce_found = true;
    //                }
    //                nonce += 1;
    //            }

    //            // Send the solved block to all other traders
    //            for peer in &m.known_traders {
    //                peer.send(b.clone()).unwrap();
    //            }
    //        }
    //    });
    //    MinerInterface {
    //        public_key: trader.public_key.clone(),
    //        transaction_sender: transaction_sender,
    //    }
    //}
}

impl MinerInterface{
    pub fn execute<F: 'static>(&self, command: F) where 
        F: FnOnce(&mut Miner) -> () + Send {
        //self.command_sender.send(Box::new(command)).unwrap();
    }
}
