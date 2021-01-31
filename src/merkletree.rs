use crate::transaction::SignedTransaction;
use super::utils::any_as_u8_slice;
use sha2::{Digest, Sha256};

enum Child<'a>{
    LeafNode(SignedTransaction<'a>),
    HashNode{
            left: Box<Child<'a>>,
            right: Option<Box<Child<'a>>>,
            hash: Vec<u8>,
    },
}

struct MerkleTree<'a>{
    root: Child<'a>,
    num_leaves: i32,
}

impl<'a> MerkleTree<'a>{
    fn new(t: SignedTransaction<'a>) -> Self{
        let leaf = Child::LeafNode(t);
        // Hash the Child node
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&leaf) };
        let hashed = Sha256::digest(&[bytes, bytes].concat()).to_vec();
        let root = Child::HashNode{
            left: Box::new(leaf), 
            right: None,
            hash: hashed,
        };
        
        MerkleTree{
            root: root,
            num_leaves: 1,
        }
    }

    fn add(&self, c: Child){
        let l = (self.num_leaves as f32).log2();
        if l == l.floor(){
             for n in 1..(l as i32){
                 let bytes: &[u8] = unsafe{ any_as_u8_slice(&c) };
                 let hashed = Sha256::digest(&[bytes, bytes].concat()).to_vec();
                 let c = Child::HashNode{
                     left: Box::new(c),
                     right: None,
                     hash: hashed,
                 }; 
             }    
        }    
    }
}

impl<'a> Child<'a>{
    fn get_depth(&self) -> i32{
        match self{
            Self::LeafNode(_t) => 0,
            Self::HashNode{left, ..} => {
                1 + left.get_depth()
            },
        }
    }
}

