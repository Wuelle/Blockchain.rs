use blockchain::trader::Trader;
use blockchain::transaction::Transaction;
use blockchain::miner::Miner;
use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    // Setup Logger
    SimpleLogger::new().init().unwrap();

    // Create core entities
    let t1 = Trader::new();
    let t2 = Trader::new();
    let m1 = Miner::new(&t1); // Miner linked to t1

    // 'me' is synonymous to 'self' since 'self' can't be assigned
    let target_key = t2.public_key.clone();
    t1.execute(move|me| {
        let t = Transaction::new(me.public_key.clone(), target_key, 1.0);
        let st = me.sign(t);
    });

    // Wait for the user to stop execution (Ctrl+C)
    loop{}
}
