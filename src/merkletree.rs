// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// All hail the Shepmaster!
use crate::utils::sha256_digest;
use sha2::{Digest, Sha256};
use log::{info, trace, warn};
use std::hash::{Hash};
use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
pub enum Node<T>
where T: Clone{
    LeafNode(T),
    HashNode{
            left: Link<T>,
            right: Link<T>,
            hash: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct MerkleTree<T: Clone>{
    pub root: Box<Node<T>>,
}

impl<T: Clone + Hash + Debug> MerkleTree<T>{
    pub fn new() -> Self {
        MerkleTree{ 
            root: Box::new(Node::HashNode{
                left: None,
                right: None,
                hash: Vec::new(),
            }),
        }
    }

    pub fn add(&mut self, value: T) {
        let c = Node::LeafNode(value);
        if !self.root.is_full(){
            self.root.add(c);
        }
        else {
            // construct a new root with branch to the right and the previous tree as the left child
            let mut new_branch = Node::HashNode{
                left: Some(Box::new(c)),
                right: None,
                hash: Vec::new(),
            };
            new_branch.set_hash();
            for _ in 1..self.root.get_depth() - 1{
                new_branch = Node::HashNode{
                    left: Some(Box::new(new_branch.clone())),
                    right: None,
                    hash: Vec::new(),
                };
                new_branch.set_hash();
            }
            let mut new_root = Node::HashNode{
                left: Some(self.root.clone()),
                right: Some(Box::new(new_branch)),
                hash: Vec::new(),
            };
            new_root.set_hash();
            self.root = Box::new(new_root);
        }
    }

    pub fn get_root_hash(&self) -> &Vec<u8> {
        match *self.root {
            Node::HashNode{ref left, ref right, ref hash} => hash,
            Node::LeafNode(_) => panic!("Merkle root node cannot be leafnode!"),
        }
    }

    /// Debugging only
    pub fn preorder_traverse(&self) -> Vec<i32>{
        let mut out = Vec::new();
        self.root.traverse_preorder(&mut out);
        out
    }
    
    /// Return the total number of nodes within the tree
    pub fn size(&self) -> i32 {
        self.root.size()
    }

    /// Return the number of leaf nodes within the tree
    pub fn len(&self) -> i32 {
        self.root.len()
    }

    pub fn get_depth(&self) -> i32 {
        self.root.get_depth()
    }

    pub fn is_valid(&self) -> bool {
        self.root.is_valid()
    }
}

impl<T: Clone + Hash + Debug> Node<T>{
    // Verify the hashes within the subtree where root is self
    pub fn is_valid(&self) -> bool {
        trace!("Validating a new node");
        match self{
            Node::HashNode{left, right, hash} => {
                trace!("its a hashnode!");
                let left_is_valid = match left {
                    Some(n) => n.is_valid(),
                    None => true,
                };
                let right_is_valid = match right {
                    Some(n) => n.is_valid(),
                    None => true,
                };
                let target = self.calc_hash();
                let i_am_valid = target == *hash;

                if !i_am_valid{
                    println!("GOT A WRONG HASH:");
                    println!("GOT: {:?}", hash);
                    println!("EXPECTED: {:?}", target);
                }
                left_is_valid && right_is_valid && i_am_valid
            },
            Node::LeafNode(_) => {println!("Its a leafnode!");true},
        }
    }
    
    pub fn add(&mut self, c: Node<T>){
        if let Node::HashNode{left, right, ..} = self{
            if let Some(ref mut leftnode) = left{
                if !leftnode.is_full(){
                    leftnode.add(c);
                    self.set_hash();
                    return
                }
            }
            else {
                *left = Some(Box::new(c));
                self.set_hash();
                return
            }
            if let Some(ref mut rightnode) = right{
                if !rightnode.is_full(){
                    rightnode.add(c);
                    self.set_hash();
                    return
                }
            }
            else{
                *right = Some(Box::new(c));
                self.set_hash();
                return
            }
        }
    }

    pub fn get_hash(&self) -> Vec<u8> {
        match self{
            Node::HashNode{left:_, right:_, hash} => {
                hash.to_vec()
            },
            Node::LeafNode(content) => {
                sha256_digest(&content)
            },
        }
    }

    pub fn set_hash(&mut self){
        let new_hash = self.clone().calc_hash();
        if let Node::HashNode{left: _, right: _, hash} = self{
            *hash = new_hash.clone();
        }
    }

    pub fn calc_hash(&self) -> Vec<u8>{
        if let Node::HashNode{left, right, hash} = self {
            let mut combined = Vec::new();

            if let Some(node) = left {
                combined.extend(&node.get_hash());
            }
            if let Some(node) = right {
                combined.extend(&node.get_hash());
            }
            // extend byte vector if necessary
            if combined.len() == 0 {
                combined = vec![0, 64];
            }
            else if combined.len() == 32 { 
                combined.extend(&combined.clone());
            }
            sha256_digest(&combined)
        }
        else{
            panic!("Calling .calc_hash() on a LeafNode doesnt make sense");
        }
    }
    
    /// Get the number of nodes in the subgraph, root not included
    pub fn size(&self) -> i32 {
        if let Node::HashNode{left, right, ..} = self{
            let mut total = 1; // The first node is 'self'
            if let Some(node) = left {
                total += node.size();
            }
            if let Some(node) = right {
                total += node.size();
            }
            total
        }
        else {
            1
        }
    }

    /// Get the number of leaf nodes in the subgraph
    pub fn len(&self) -> i32 {
        if let Node::HashNode{left, right, ..} = self{
            let mut total = 0;
            if let Some(node) = left {
                total += node.len();
            }
            if let Some(node) = right {
                total += node.len();
            }
            total
        }
        else {
            1
        }
    }
    
    pub fn get_depth(&self) -> i32{
        match self {
            Node::LeafNode(_) => 1,
            Node::HashNode{left, ..} => {
                if let Some(node) = left {
                    1 + node.get_depth()
                }
                else {
                    1
                }
            }
        }
    }

    /// preorder traversing(debugging only)
    pub fn traverse_preorder(&self, out: &mut Vec<i32>){
        match self {
            Node::HashNode{left, right, ..} => {
                out.push(1);
                match left {
                    Some(x) => x.traverse_preorder(out),
                    None => out.push(-1),
                }
                match right {
                    Some(x) => x.traverse_preorder(out),
                    None => out.push(-1),
                }
            },
            Node::LeafNode(_) => { out.push(0) },
        }
    }

    /// Whether or not more nodes can be added to the subtree
    pub fn is_full(&self) -> bool{
        match self{
            Node::LeafNode(_) => true,
            Node::HashNode{left, right, ..} => {
                let r_full = match right{
                    Some(node) => node.is_full(),
                    None => false
                };
                let l_full = match left{
                    Some(node) => node.is_full(),
                    None => false
                };
                l_full && r_full
            }
        }
    }
}

#[cfg(test)]
mod test{
    // Imports
    use super::*;

    #[test]
    fn build_tree(){
        let mut tree = MerkleTree::new();

        // Add some nodes for testing
        for index in 1..10 {
            tree.add(index);
            println!("{:?}", tree.preorder_traverse());
            println!("{:?} != {:?}", tree.len(), index);
            assert!(tree.len() == index);
            assert!(tree.size() > tree.len());
        }
    }

    #[test]
    fn validate_tree() {
        let mut tree = MerkleTree::new();

        // Fill the tree
        for index in 1..10 {
            tree.add(index);
            assert!(tree.is_valid());
        }

        // Invalidate the tree by modifying the root node's hash
        if let Node::HashNode{ref left, ref right, ref mut hash} = *tree.root {
            let new_hash = vec![1, 2, 3, 4];
            *hash = new_hash;
        }
        else {
            panic!("Root node is not a HashNode!");
        }
        assert!(!tree.is_valid());
    }


}
