use log::{info, trace, warn};
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme, Hash};
use rand::rngs::OsRng;
use crate::blockchain::{Block, Blockchain};
use crate::transaction::{Transaction, SignedTransaction};
use crate::utils::sha256_digest;
use crate::miner::MinerInterface;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, Sender};

// a closure to be sent to the trader thread, to enable the user to control trader behaviour from main()
type Command = Box<dyn FnOnce(&mut Trader) -> () + Send>;
type BlockSender = Sender<Block>;

pub struct Trader {
    pub public_key: RSAPublicKey,
    private_key: RSAPrivateKey,
    pub known_miners: Vec<Sender<SignedTransaction>>, // Contains channels to every other miner
    blockchain: Blockchain,
}

pub struct TraderInterface {
    pub public_key: RSAPublicKey,
    command_sender: Sender<Command>,
    pub block_sender: Sender<Block>,
}

impl Trader{
    pub fn new() -> TraderInterface {
        // Generate a random 256bit RSA key pair
        let mut rng = OsRng;
        let private_key = RSAPrivateKey::new(&mut rng, 512).expect("Failed to generate a key");
        let public_key = RSAPublicKey::from(&private_key);

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
        info!("Spawning a new Trader thread!");

        thread::spawn(move|| {
            loop {
                // Check for new transactions to be added to the blockchain
                match block_receiver.try_recv() {
                    Ok(block) => {
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
                        c(&mut self);
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
}

impl TraderInterface {
    pub fn execute<F: 'static>(&self, command: F) where 
        F: FnOnce(&mut Trader) -> () + Send {
        self.command_sender.send(Box::new(command)).unwrap();
    }

    pub fn add_miner(&self, miner: &MinerInterface) {
        let sender = miner.transaction_sender.clone();
        self.execute(move|me: &mut Trader| {
            me.known_miners.push(sender);
        });
    }

    pub fn add_trader(&self) {
    }
}
