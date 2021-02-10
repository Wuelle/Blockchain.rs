use crossbeam::channel::{unbounded, Sender, Receiver};
use blockchain::trader::Trader;
use blockchain::blockchain::{Block};
use blockchain::transaction::{Transaction, SignedTransaction};
use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Create channels for broadcasting transactions/mined blocks
    let (st, rt): (Sender<SignedTransaction>, Receiver<SignedTransaction>) = unbounded();
    let (sb, rb): (Sender<Block>, Receiver<Block>) = unbounded();
    let mut miner_threads = Vec::new();
    

    let t1 = Trader::new();
    let t2 = Trader::new();

    trace!("Spawning Miner Thread!");
    miner_threads.push(t1.spawn_miner_thread(rt.clone(), sb.clone()));
    //miner_threads.push(t2.spawn_miner_thread(rt.clone(), sb.clone()));

    // Perform an a Transaction
    trace!("Creating Transaction");
    let trans1 = Transaction::new(t1.public_key.clone(), t2.public_key.clone(), 100.0);
    trace!("Signing Transaction!");
    let strans1 = t2.sign(trans1);
    
    // Broadcast new transaction to the miners
    trace!("Broadcasting Transaction!");
    st.send(strans1.clone()).unwrap();

    info!("Waiting for the miner to return the mined block");
    let b = rb.recv().unwrap();
    info!("Got block, the nonce is {:?}", b.nonce);
    
    // Wait for the Miner threads to complete any leftover work
    //for handle in miner_threads{
    //    handle.join().unwrap();
    //}
}
