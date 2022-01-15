use super::*;
use std::mem;

pub mod hash_structures;
use hash_structures::*;

const HASH_LENGTH: usize = 256;
const DEFAULT_HASH: &[u8] = &[1; 32];

pub(super) struct TreeNode {
    entry: NodeKind,
    version: Option<Version>,
    pub(super) left: Option<Rc<RefCell<TreeNode>>>,
    pub(super) right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub(super) fn new() -> Self {
        TreeNode {
            entry: NodeKind::Unassigned,
            version: None,
            left: None,
            right: None,
        }
    }
    //TODO take care of ccar
    pub(super) fn new_leaf(hashbytes: HashBytes) -> Self {
        TreeNode {
            version: Some(Version::new(1, &hashbytes)),
            entry: NodeKind::Leaf(hashbytes),
            left: None,
            right: None,
        }
    }

    pub(super) fn key_hash_raw(&self) -> &Vec<u8> {
        match self.entry {
            NodeKind::Leaf(ref hashbytes) => &hashbytes.bytes(),
            NodeKind::Inner(ref hashbytes) => &hashbytes.bytes(),
            _ => panic!("NodeKind::Unassigned does not contain hashbytes"),
        }
    }

    pub(super) fn version_hash_raw(&self) -> &Vec<u8> {
        match self.version {
            Some(ref version) => version.hash.bytes(),
            None => self.key_hash_raw(),
        }
    }

    pub(super) fn insert<T: Hashable + Debug>(&mut self, key: T) {
        let hashbits = key.hash();
        self.insert_with_level(hashbits, 0);
    }

    fn update_version(&mut self) {
        if let Some(ref mut vers) = self.version {
            vers.value += 1;
            vers.hash = hash_from_version(vers.value, self.entry.get_hash());
        } else {
            panic!("version not available to update")
        }
    }

    fn get_version(&self) -> Option<i32> {
        if let Some(ref vers) = self.version {
            Some(vers.value)
        } else {
            None
        }
    }

    fn new_from_luggage(luggage: (NodeKind, Option<Version>)) -> Self {
        TreeNode {
            entry: luggage.0,
            version: luggage.1,
            left: None,
            right: None,
        }
    }

    fn insert_with_level(&mut self, hash_to_insert: HashBytes, level: usize) {
        // Choose left or right
        let (next_child, _) = next_child(&mut self.left, &mut self.right, &hash_to_insert, level);

        match next_child {
            Some(ref node) => {
                // Next is non-empty
                node.borrow_mut()
                    .insert_with_level(hash_to_insert, level + 1);
            }
            None => {
                // We are in a leaf equal to which we want to add
                if let NodeKind::Leaf(ref curhash) = self.entry {
                    if curhash.bytes() == hash_to_insert.bytes() {
                        self.update_version();
                        return;
                    }
                }
                if self.entry.is_leaf() {
                    // We are in a leaf. Move the hash out of the self.entry and replace it with an unassigned NodeKind
                    let entry = mem::replace(&mut self.entry, NodeKind::Unassigned);
                    let version = self.version.take();
                    self.insert_with_luggage(hash_to_insert, level, (entry, version));
                } else {
                    // We are in an Inner node and next is None
                    *next_child = Some(Rc::new(RefCell::new(TreeNode::new_leaf(hash_to_insert))));
                }
            }
        }
        // Backtrack
        let new_hashbytes = hash_from_children(&self.left, &self.right);
        self.entry = NodeKind::Inner(new_hashbytes);
    }

    fn insert_with_luggage(
        &mut self,
        hash_to_insert: HashBytes,
        level: usize,
        luggage: (NodeKind, Option<Version>),
    ) {
        let (next_child, alt_child) =
            next_child(&mut self.left, &mut self.right, &hash_to_insert, level);

        //If there is a luggage (next has to be empty)
        if luggage.0.get_hash().bit(level) != hash_to_insert.bit(level) {
            *next_child = Some(Rc::new(RefCell::new(TreeNode::new_leaf(hash_to_insert))));
            *alt_child = Some(Rc::new(RefCell::new(TreeNode::new_from_luggage(luggage))));
            //*alt_child = Some(Rc::new(RefCell::new(TreeNode::new_leaf(luggage))));
        } else {
            let mut nextnode = TreeNode::new();
            nextnode.insert_with_luggage(hash_to_insert, level + 1, luggage);
            *next_child = Some(Rc::new(RefCell::new(nextnode)));
        }
        // Backtrack
        let new_hashbytes = hash_from_children(&self.left, &self.right);
        self.entry = NodeKind::Inner(new_hashbytes);
    }

    pub(super) fn contains<T: Hashable + Debug>(&self, key: T) -> Option<i32> {
        let hash_bits = key.hash();
        let mut proof = vec![];
        self.contains_hash(&hash_bits, 0,  &mut proof)
    }

    pub(super) fn get_proof<T: Hashable + Debug>(&self, key: T) -> Vec<ProofNode> {
        let hash_bits = key.hash();
        let mut proof = vec![];
        self.contains_hash(&hash_bits, 0,  &mut proof);
        proof
    }

    fn contains_hash(&self, hash_to_check: &HashBytes, level: usize, proof: &mut Vec<ProofNode>) -> Option<i32> {
        if level >= HASH_LENGTH {
            //This will happen with almost 0 probability
            panic!();
        }

        if let NodeKind::Leaf(hashbytes) = &self.entry {
            proof.push(ProofNode::Leaf(self.key_hash_raw().clone(), self.get_version().unwrap()));
            if hashbytes.bytes() == hash_to_check.bytes() {
                return self.get_version();
            } else {
                return None;
            }
        }

        let (next_child, alt_child) = next_child(&self.left, &self.right, &hash_to_check, level);

        let branch_hash = if let Some(alt_child) = alt_child {
            alt_child.borrow().version_hash_raw().clone()
        } else {
            DEFAULT_HASH.to_vec()
        };

        if hash_to_check.bit(level) == 1 {
            proof.push(ProofNode::Left(branch_hash));
        } else {
            proof.push(ProofNode::Right(branch_hash));
        }

        match next_child {
            Some(ref node) => node.borrow().contains_hash(hash_to_check, level + 1, proof),
            None => {
                proof.push(ProofNode::None);
                None
            },
        }
    }
}

fn next_child<T>(left: T, right: T, hash: &HashBytes, level: usize) -> (T, T) {
    if hash.bit(level) == 0 {
        (left, right)
    } else {
        (right, left)
    }
}

pub fn hash_from_version(version: i32, key_hash: &HashBytes) -> HashBytes {
    // Transform i32 to vector of bytes.
    let mut bytes: Vec<u8> = vec![];
    let mask = (1 << 8) - 1;
    for i in 0..4 {
        bytes.push((version >> (i * 8) & mask) as u8);
    }
    //Attach bytes to key_hash and hash
    bytes.extend(key_hash.bytes());
    bytes.hash()
}

//TODO replace digest with Hashable trait functions
fn hash_from_children(
    node1: &Option<Rc<RefCell<TreeNode>>>,
    node2: &Option<Rc<RefCell<TreeNode>>>,
) -> HashBytes {
    let mut to_hash: Vec<u8>;
    match node1 {
        Some(node) => {
            let temp = node.borrow();
            to_hash = temp.version_hash_raw().clone();
        }
        None => {
            to_hash = DEFAULT_HASH.to_vec();
        }
    }
    match node2 {
        Some(node) => {
            to_hash.extend(node.borrow().version_hash_raw());
        }
        None => {
            to_hash.extend(DEFAULT_HASH);
        }
    }
    to_hash.hash()
}
#[derive(Debug)]
pub enum ProofNode {
    Left(Vec<u8>),
    Right(Vec<u8>),
    Leaf(Vec<u8>, i32),
    None,
}

impl ProofNode {
    fn add_and_hash(self, vec_to_add: &mut Vec<u8>) -> Vec<u8> {
        let to_hash = match self {
            Self::Left( mut v) => {
                v.extend(vec_to_add.clone());
                v
            },
            Self::Right(v) => {
                vec_to_add.extend(&v);
                vec_to_add.clone()
            },
            _ => panic!("combining proofnode hashes"),
        };
        to_hash.hash().bytes().clone()
    }

    fn hash(self) -> Vec<u8>{
        let to_hash = match self {
            Self::Leaf(key_hash,version) => {
                hash_from_version(version, &HashBytes::new(key_hash)).bytes().clone()
            },
            Self::None => {
                DEFAULT_HASH.to_vec()
            },
            _ => panic!("hashing proofnode"),
        };
        to_hash
    }
}

pub fn roothash_from_proof(proof: &mut Vec<ProofNode>) -> Vec<u8> {
    let mut ans = proof.pop().unwrap().hash();
    while let Some(node) = proof.pop() {
        ans = node.add_and_hash(&mut ans);
        //println!("ans {:?}", ans);
    }
    ans
}