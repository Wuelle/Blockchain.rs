// Local imports
use blockchain::trader::{Trader};
use blockchain::transaction::{Transaction, SignedTransaction};
use blockchain::merkletree::Node;

fn main() {
    let t1 = Trader::new();
    let t2 = Trader::new();

    // Perform an a Transaction
    let trans1 = Transaction::new(&t1, &t2, 100);
    let strans1 = t2.sign(&trans1);
    
    let trans2 = Transaction::new(&t1, &t2, 200);
    let strans2 = t1.sign(&trans2);

    // build a tree
    let node1 = Node::new(strans1);

    println!("{:?}", node1.get_depth());
}
