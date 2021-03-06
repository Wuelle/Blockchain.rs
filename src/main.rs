use blockchain::trader::Trader;
use blockchain::transaction::Transaction;
use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    // Setup Logger
    SimpleLogger::new().init().unwrap();

    // Every Trader gets a copy of st and distributes copies of his sb
    let mut miners = Vec::new(); // Bad variable name!
    let mut traders = Vec::new();
    let mut trader_commands = Vec::new();

    Trader::new(true, &mut miners, &mut traders, &mut trader_commands);
    Trader::new(false, &mut miners, &mut traders, &mut trader_commands);

    // 'me' is synonymous to 'self' since 'self' can't be assigned
    let x = 3;
    trader_commands[0].send(Box::new(move |me| {
        println!("I AM EXECUTING A COMMAND! {:?}", x);
    })).unwrap();

    // Broadcast new transaction to the miners
    //trace!("Broadcasting Transaction to {:?} Miners!", miners.len());
    //for sender in miners{
    //     sender.send(strans1.clone()).unwrap();
    //}
    // Wait until user stops all threads (Ctrl+C)
    loop{}
    // Wait for the Miner threads to complete any leftover work
    //for handle in miners{
    //    handle.join().unwrap();
    //}
}
