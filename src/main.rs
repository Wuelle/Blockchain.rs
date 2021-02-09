use crossbeam::channel::{unbounded, Sender, Receiver};
use blockchain::trader::Trader;
use blockchain::transaction::{Transaction, SignedTransaction};
use log::{info, trace, warn};
use simple_logger::SimpleLogger;
use std::sync::Arc;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Create channel for broadcasting transmissions
    let (s, r): (Sender<Arc<SignedTransaction>>, Receiver<Arc<SignedTransaction>>) = unbounded();
    let mut miner_threads = Vec::new();
    

    let t1 = Arc::new(Trader::new());
    let t2 = Arc::new(Trader::new());

    trace!("Spawning Miner Thread!");
    miner_threads.push(t1.spawn_miner_thread(r.clone()));

    // Perform an a Transaction
    trace!("Creating Transaction");
    let trans1 = Transaction::new(t1.clone(), t2.clone(), 100.0);
    trace!("Signing Transaction!");
    let strans1 = Arc::new(t2.sign(trans1));
    
    // Broadcast new transaction to the miners
    trace!("Broadcasting Transaction!");
    s.send(strans1).unwrap();

    for handle in miner_threads{
        handle.join();
    }
}
