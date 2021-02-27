use blockchain::trader::Trader;
use blockchain::blockchain::{Block};
use blockchain::transaction::{Transaction, SignedTransaction};
use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Every Trader gets a copy of st and distributes copies of his sb
    let mut miners = Vec::new(); // Bad variable name!
    let mut traders = Vec::new();

    let t1 = Trader::new(||{
        println!("HI FROM THE MINER THREAD!");
    }, true, &mut miners, &mut traders);
    let t2 = Trader::new(||{}, false, &mut miners, &mut traders);

    // Perform an a Transaction
    trace!("Creating Transaction");
    let trans1 = Transaction::new(t1.public_key.clone(), t2.public_key.clone(), 100.0);
    trace!("Signing Transaction!");
    let strans1 = t2.sign(trans1);
    
    // Broadcast new transaction to the miners
    trace!("Broadcasting Transaction to {:?} Miners!", miners.len());
    for sender in miners{
         sender.send(strans1.clone()).unwrap();
    }

    // Wait for the Miner threads to complete any leftover work
    //for handle in miners{
    //    handle.join().unwrap();
    //}
}
