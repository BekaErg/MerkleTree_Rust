use crypto_hash::{digest, Algorithm};

pub trait Hashable {
    fn hash(&self) -> HashBytes;
    fn raw_hash(&self) -> Vec<u8>;
}

impl Hashable for i32 {
    fn hash(&self) -> HashBytes {
        let s = self.to_string();
        HashBytes::new(digest(Algorithm::SHA256, s.as_bytes()))
    }
    fn raw_hash(&self) -> Vec<u8> {
        let s = self.to_string();
        digest(Algorithm::SHA256, s.as_bytes())
    }
}

impl Hashable for usize {
    fn hash(&self) -> HashBytes {
        let s = self.to_string();
        HashBytes::new(digest(Algorithm::SHA256, s.as_bytes()))
    }
    fn raw_hash(&self) -> Vec<u8> {
        let s = self.to_string();
        digest(Algorithm::SHA256, s.as_bytes())
    }
}

impl Hashable for Vec<u8> {
    fn hash(&self) -> HashBytes {
        HashBytes::new(digest(Algorithm::SHA256, self))
    }
    fn raw_hash(&self) -> Vec<u8> {
        digest(Algorithm::SHA256, self)
    }
}


pub(super) enum NodeKind {
    Leaf(HashBytes),
    Inner(HashBytes),
    Unassigned,
}

impl NodeKind {
    pub(super) fn is_leaf(&self) -> bool {
        match self {
            NodeKind::Leaf(_) => true,
            _ => false,
        }
    }

    pub(super) fn get_hash(&self) -> &HashBytes {
        match self {
            NodeKind::Leaf(hashbytes) => hashbytes,
            NodeKind::Inner(hashbytes) => hashbytes,
            NodeKind::Unassigned => panic!("node does not contain hash"),
        }
    }
}

pub(super) struct Version {
    pub(super) value: i32,
    pub(super) hash: HashBytes,
}

impl Version {
    pub(super) fn new(value: i32, key_hash: &HashBytes) -> Self{
        Self {
            value,
            hash: super::hash_from_version(value, key_hash),
        }
    }

}

#[derive(Debug)]
pub struct HashBytes {
    bytes: Vec<u8>,
}

impl HashBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        HashBytes { bytes }
    }

    //pub(super)
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub(super) fn bit(&self, index: usize) -> u8 {
        let (byte_index, bit_index) = (index >> 3, index & 7);
        (self.bytes[byte_index] >> (7 - bit_index)) & 1
    }
}
