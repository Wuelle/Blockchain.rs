use crate::transaction::SignedTransaction;
use super::utils::any_as_u8_slice;
use sha2::{Digest, Sha256};

fn concat_slice<'a>(b1: &'a[u8], b2: &'a[u8]) -> &'a[u8]{
    let vec1 = b1.iter().collect::<Vec<_>>();
    let vec2 = b2.iter().collect::<Vec<_>>();
    &[b1, b2].concat()
}

enum Child<'a, T>{
    LeafNode(T),
    HashNode{
            left: Box<Child<'a, T>>,
            right: Option<Box<Child<'a, T>>>,
            hash: &'a [u8],
    },
}

struct MerkleTree<'a, T>{
    root: Child<'a, T>,
    num_leaves: i32,
}

impl<'a, T> MerkleTree<'a, T>{
    fn new(t: SignedTransaction) -> Self{
        let leaf = Child::LeafNode(t);
        // Hash the Child node
        let bytes: &[u8] = unsafe{ any_as_u8_slice(&t) };
        let concat = concat_slice(bytes, bytes); 
        let hashed = Sha256::digest(&concat).to_vec();
        let root = Child::HashNode{
            left: Box::new(leaf), 
            right: None,
            hash: &hashed,
        };
        
        MerkleTree{
            root: root,
            num_leaves: 1,
        }
    }
}

impl<'a, T> Child<'a, T>{
    fn get_depth(&self) -> i32{
        match self{
            Self::LeafNode(_t) => 0,
            Self::HashNode{left, ..} => {
                1 + left.get_depth()
            },
        }
    }
}

