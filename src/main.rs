// Local imports
use blockchain::trader::{Trader};
use blockchain::transaction::{Transaction, SignedTransaction};
use blockchain::merkletree::*;

fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();

    // Perform an a Transaction
    let t = Transaction::new(&t1, &t2, 100);
    let st: SignedTransaction = t2.sign(&t);
    println!("{:?}", st.verify());
}
