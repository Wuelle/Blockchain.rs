use blockchain::trader::Trader;
use blockchain::transaction::Transaction;
use log::{info, trace, warn};
use simple_logger::SimpleLogger;

fn main() {
    // Setup Logger
    SimpleLogger::new().init().unwrap();

    let mut miners = Vec::new(); // Bad variable name!
    let mut traders = Vec::new();

    let t1 = Trader::new(true, &mut miners, &mut traders);
    let t2 = Trader::new(false, &mut miners, &mut traders);

    // 'me' is synonymous to 'self' since 'self' can't be assigned
    let target_key = t2.public_key.clone();
    t1.execute(move|me| {
        println!("I am now starting a new transaction!");
        let t = Transaction::new(me.public_key.clone(), target_key, 1.0);
        let st = me.sign(t);
        println!("is_valid: {:?}", st.is_valid());
    });
    loop{}
}
