use std::cell::RefCell;
use std::rc::Rc;
use treenode::*;
use treenode::hash_structures::*;
use std::fmt::Debug;

pub struct MerkleTree {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl MerkleTree {
    pub fn new() -> Self {
        MerkleTree { root: None }
    }
    pub fn contains<T: Hashable + Debug>(&mut self, key: T) -> Option<i32>{
        match self.root {
            Some(ref node) => {
                node.borrow().contains(key)
            },
            None => {
                None
            },
        }
    }

    pub fn get_proof<T: Hashable + Debug>(&mut self, key: T) -> Vec<ProofNode>{
        match self.root {
            Some(ref node) => {
                node.borrow().get_proof(key)
            },
            None => {
                panic!();
            },
        }
    }

    pub fn insert<T: Hashable + Debug>(&mut self, key: T) {
        match self.root {
            Some(ref node) => {
                node.borrow_mut().insert(key);
            }
            None => {
                *self = MerkleTree {
                    root: Some( Rc::new(RefCell::new(TreeNode::new_leaf(key.hash())))),
                }
            }
        }
    }

    pub fn get_hash(&self) -> Option<Vec<u8>> {
        match self.root {
            Some(ref node) => Some(node.borrow().version_hash_raw().clone()),
            None => None,
        }
    }

}


mod debug_functions;
mod treenode;



#[cfg(test)]
mod unit_tests;







