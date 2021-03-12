use blockchain::trader::Trader;
use blockchain::transaction::Transaction;
use simple_logger::SimpleLogger;

fn main() {
    // Setup Logger
    SimpleLogger::new().init().unwrap();

    // Create core entities
    let mut t1 = Trader::new();
    let mut t2 = Trader::new();
    let m1 = t1.spawn_miner_thread();

    // Create links for p2p network
    t1.link(&mut t2);
    t1.register_miner(&m1);
    t2.register_miner(&m1);

    let t = Transaction::new(t1.public_key.clone(), t2.public_key.clone(), 1.0);
    let st = t1.sign(t);
    t1.broadcast(&st);

    // Wait for the user to stop execution (Ctrl+C)
    loop{}
}
