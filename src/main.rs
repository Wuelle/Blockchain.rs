// Local imports
use blockchain::trader::{Trader};
use blockchain::transaction::{Transaction};
//use blockchain::merkletree::Node;

fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();

    // Perform an a Transaction
    let trans1 = Transaction::new(&t1, &t2, 100.0);
    let strans1 = t2.sign(&trans1);
    
    println!("{:?}", strans1.is_valid());
}
