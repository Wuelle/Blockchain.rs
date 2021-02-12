// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// All hail the Shepmaster!
use crate::utils::sha256_digest;
use log::{info, trace, warn};

type Link<T> = Box<Node<T>>;

#[derive(Clone)]
pub enum Node<T>
where T: Clone{
    LeafNode(T),
    HashNode{
            left: Option<Link<T>>,
            right: Option<Link<T>>,
            hash: Vec<u8>,
    },
}

#[derive(Clone)]
pub struct MerkleTree<T: Clone>{
    root: Link<T>,
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
            for _ in 1..self.root.get_depth() - 1{
                new_branch = Node::HashNode{
                    left: Some(Box::new(new_branch.clone())),
                    right: None,
                    hash: Vec::new(),
                };
            }
            trace!("The right branch contains {} new nodes!({})", new_branch.size(), self.root.get_depth());
            let new_root = Node::HashNode{
                left: Some(self.root.clone()),
                right: Some(Box::new(new_branch)),
                hash: Vec::new(),
            };
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
    pub fn new(t: T) -> Self{
        let leaf = Node::LeafNode(t);
        
        // Hash the Node
        let hashed = sha256_digest(&leaf);
        Node::HashNode{
            left: Some(Box::new(leaf)), 
            right: None,
            hash: hashed,
        }    
    }

    pub fn add(&mut self, c: Node<T>){
        if let Node::HashNode{left, right, ..} = self{
            if let Some(ref mut leftnode) = left{
                if !leftnode.is_full(){
                    leftnode.add(c);
                    return
                }
            }
            else {
                *left = Some(Box::new(c));
                return
            }
            if let Some(ref mut node) = right{
                if !node.is_full(){
                    node.add(c);
                    return
                }
            }
            else{
                *right = Some(Box::new(c));
                return
            }
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
