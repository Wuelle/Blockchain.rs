use crate::transaction::SignedTransaction;
use super::utils::any_as_u8_slice;
use sha2::{Digest, Sha256};

pub enum Node<'a>{
    LeafNode(SignedTransaction<'a>),
    HashNode{
            left: Box<Node<'a>>,
            right: Option<Box<Node<'a>>>,
            hash: Vec<u8>,
    },
}

impl<'a> Node<'a>{
    pub fn new(t: SignedTransaction<'a>) -> Self{
        let leaf = Node::LeafNode(t);
        
        // Hash the Node
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&leaf) };
        let hashed = Sha256::digest(&[bytes, bytes].concat()).to_vec();
        Node::HashNode{
            left: Box::new(leaf), 
            right: None,
            hash: hashed,
        }    
    }

    pub fn add(&mut self, mut c: Node<'a>){
        if let Node::HashNode{left, mut right, ..} = self {
            if !left.is_full(){
                left.add(c);
                return
            }
            else if let Some(mut right) = right.as_ref(){
                if !right.is_full() {
                    right.add(c);
                    return
                } 
            } else {
                // Right is empty and left is full, add to right as new branch
                let depth = self.get_depth();
                for _ in 0..depth{
                    let bytes: &[u8] = unsafe{ any_as_u8_slice(&c) };
                    let hashed = Sha256::digest(&[bytes, bytes].concat()).to_vec();
                    c = Node::HashNode{
                        left: Box::new(c),
                        right: None,
                        hash: hashed,
                    };
                }
                right = Some(Box::new(c));
            }
        }
        else{
            println!("Cannot add nodes to a LeafNode - call add() on the graph root!");
        };
    }
    
    pub fn get_depth(&self) -> i32{
        match self{
            Self::LeafNode(_t) => 0,
            Self::HashNode{left, ..} => {
                1 + left.get_depth()
            },
        }
    }

    pub fn is_full(&self) -> bool{
        match self{
            Node::LeafNode(_) => true,
            Node::HashNode{left, right, ..} => {
                let r_full = match right{
                    Some(node) => node.is_full(),
                    None => false
                };
                let l_full = left.is_full();
                l_full && r_full
            }
        }
    }
}
