use blockchain::trader::Trader;
use blockchain::transaction::Transaction;
use blockchain::miner::Miner;
use simple_logger::SimpleLogger;

fn main() {
    // Setup Logger
    SimpleLogger::new().init().unwrap();

    // Create core entities
    let t1 = Trader::new();
    let t2 = Trader::new();
    let m1 = t1.spawn_miner_thread();

    t1.register_miner(&m1);
    t2.register_miner(&m1);

    //let m1 = Miner::new(&t1); // Miner linked to t1

    // Link Miners/Traders to create P2P network
    //t1.add_miner(&m1);
    //t2.add_miner(&m1);

    // 'me' is synonymous to 'self' since 'self' can't be assigned
    let t = Transaction::new(t1.public_key.clone(), t2.public_key.clone(), 1.0);
    let st = t1.sign(t);
    t1.broadcast(&st);


    // Wait for the user to stop execution (Ctrl+C)
    loop{}
}
