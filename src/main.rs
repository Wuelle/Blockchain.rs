use crossbeam::channel::unbounded;
use blockchain::trader::{Trader};
use blockchain::transaction::{Transaction, SignedTransaction};
//use blockchain::merkletree::Node;

fn main() {
    // Create channel for broadcasting transmissions
    let (s, r) = unbounded();
    
    let miner_threads = Vec::new();

    let t1: Trader = Trader::new();
    let t2: Trader = Trader::new();

    miner_threads.push(t1.spawn_miner_thread(r.clone()));

    // Perform an a Transaction
    let trans1: Transaction = Transaction::new(&t1, &t2, 100.0);
    let strans1: SignedTransaction = t2.sign(trans1);
    
    // Broadcast new transaction to the miners
    s.send(&strans1).unwrap();
    
    // Join all the miner threads back into main
    for t in miner_threads{
        t.join().expect("Miner thread panicked!");
    }
}
