// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// All hail the Shepmaster!
use crate::utils::sha256_digest;
use sha2::{Digest, Sha256};
use log::{info, trace, warn};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub enum Node<T>
where T: Clone{
    LeafNode(T),
    HashNode{
            left: Link<T>,
            right: Link<T>,
            hash: Vec<u8>,
    },
}

#[derive(Clone)]
pub struct MerkleTree<T: Clone>{
    pub root: Box<Node<T>>,
}

impl<T: Clone> MerkleTree<T>{
    pub fn new() -> Self {
        MerkleTree{ 
            root: Box::new(Node::HashNode{
                left: None,
                right: None,
                hash: Vec::new(),
            }),
        }
    }

    pub fn add(&mut self, c: Node<T>) {
        if !self.root.is_full(){
            self.root.add(c);
        }
        else {
            // Construct a new root with branch to the right and the previous tree as the left child
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
    
    /// Return the total number of nodes within the tree
    pub fn size(&self) -> i32 {
        self.root.size()
    }

    pub fn get_depth(&self) -> i32 {
        self.root.get_depth()
    }
}

impl<T: Clone> Node<T>{
    //pub fn new(t: T) -> Self{
    //    info!("NOOOO");
    //    
    //    // Hash the Node
    //    let hashed = sha256_digest(&leaf);
    //    Node::HashNode{
    //        left: Some(Box::new(leaf)), 
    //        right: None,
    //        hash: hashed,
    //    }    
    //}

    // Verify the hashes within the subtree where root is self
    pub fn is_valid(&self) -> bool {
        match self{
            Node::HashNode{left, right, hash} => {
                let left_is_valid = match left {
                    Some(n) => n.is_valid(),
                    None => true,
                };
                let right_is_valid = match right {
                    Some(n) => n.is_valid(),
                    None => true,
                };
                let i_am_valid = hash
                    .iter()
                    .zip(self.calc_hash())
                    .all(|(a, b)| *a == b);

                if !i_am_valid{
                    println!("GOT A WRONG HASH:");
                    println!("GOT: {:?}", hash);
                    println!("EXPECTED: {:?}", self.calc_hash());
                }
                left_is_valid && right_is_valid && i_am_valid
            },
            Node::LeafNode(_) => {true},
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
            *hash = new_hash;
        }
    }

    pub fn calc_hash(&self) -> Vec<u8>{
        if let Node::HashNode{left, right, hash} = self {
            let mut combined = Vec::new();

            if let Some(node) = left {
                combined.extend(node.get_hash());
            }
            if let Some(node) = right {
                combined.extend(node.get_hash());
            }
            // extend byte vector if necessary
            if combined.len() == 0 {
                combined = vec![0, 64];
            }
            else if combined.len() == 32 { 
                combined.extend(&combined.clone());
            }

            Sha256::digest(&combined).to_vec()
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
