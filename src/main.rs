use crossbeam::channel::unbounded;
use blockchain::trader::{Trader};
use blockchain::transaction::{Transaction};
//use blockchain::merkletree::Node;

fn main() {
    // Create channel for broadcasting transmissions
    let (s, r) = unbounded();

    let t1 = Trader::new();
    let t2 = Trader::new();

    t1.spawn_miner_thread(r.clone());

    // Perform an a Transaction
    let trans1 = Transaction::new(&t1, &t2, 100.0);
    let strans1 = t2.sign(trans1);
    
    //let trans2 = Transaction::new(&t1, &t2, 200);
    //let strans2 = t1.sign(&trans2);

    //// build a tree
    //let node1 = Node::new(strans1);

    //println!("{:?}", node1.get_depth());

    // test transmission
    for i in 0..5{
        s.send(i).unwrap();
    }
}
