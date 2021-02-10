use crossbeam::channel::{unbounded, Sender, Receiver};
use blockchain::trader::Trader;
use blockchain::blockchain::{Block};
use blockchain::transaction::{Transaction, SignedTransaction};
use log::{info, trace, warn};
use simple_logger::SimpleLogger;
use std::sync::Arc;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Create channels for broadcasting transactions/mined blocks
    let (st, rt): (Sender<Arc<SignedTransaction>>, Receiver<Arc<SignedTransaction>>) = unbounded();
    let (sb, rb): (Sender<Arc<Block>>, Receiver<Arc<Block>>) = unbounded();
    let mut miner_threads = Vec::new();
    

    let t1 = Trader::new();
    let t2 = Trader::new();

    trace!("Spawning Miner Thread!");
    miner_threads.push(t1.spawn_miner_thread(rt.clone(), sb.clone()));

    // Perform an a Transaction
    trace!("Creating Transaction");
    let trans1 = Transaction::new(t1.clone(), t2.clone(), 100.0);
    trace!("Signing Transaction!");
    let strans1 = Arc::new(t2.sign(trans1));
    
    // Broadcast new transaction to the miners
    trace!("Broadcasting Transaction!");
    st.send(strans1).unwrap();

    info!("Waiting for the miner to return the mined block");
    let b = rb.recv().unwrap();
    info!("Got block, the nonce is {:?}", b.nonce);

    for handle in miner_threads{
        handle.join().unwrap();
    }
}
