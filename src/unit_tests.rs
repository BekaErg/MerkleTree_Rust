use super::*;
use rand::prelude::*;
//extern crate rand;
//extern crate rand_chacha;
//use rand::{Rng, SeedableRng};
use debug_functions;
extern crate criterion;

#[test]
fn insert_contains() {
    let mut testree = MerkleTree::new();
    let v = rand_int_vec(1u64, 500, 20);
    // add vertices
    for i in 0..v.len() {
        //assert!(!testree.contains(i));
        for _ in 0..v[i] {
            testree.insert(i);
        }
    }

    //check
    for i in 0..v.len() {
        if v[i]>0 {
            assert_eq!(testree.contains(i).unwrap(), v[i]);
        } else {
            assert!(testree.contains(i) == None);
        }
    }
}



#[test]
fn proof_contains() {
    let mut testree = MerkleTree::new();
    let v = rand_int_vec(1u64, 500, 20);
    // add vertices
    for i in 0..v.len() {
        //assert!(!testree.contains(i));
        for _ in 0..v[i] {
            testree.insert(i);
        }
    }

    //check
    for i in 0..v.len() {
        if v[i]>0 {
            assert_eq!(testree.contains(i).unwrap(), v[i]);
        } else {
            assert!(testree.contains(i) == None);
        }
        let proof_root_hash = roothash_from_proof(&mut testree.get_proof(i));
        assert_eq!(testree.get_hash().unwrap(), proof_root_hash);
    }


}



#[test]
fn root_hash_invariance() {
    let (mut testree1, mut testree2) = (MerkleTree::new(), MerkleTree::new());
    let m = 40;
    let mut v: Vec<i32> = rand_int_vec(7u64, 400, m );

    for i in 0..v.len() {
        for _ in 0..v[i] {
            testree1.insert(v[i]);
        }
    }

    //shuffle range
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(2);
    v.shuffle(&mut rng);

    for j in 0..m+1 {
        v.shuffle(&mut rng);
        for i in 0..v.len() {
            if v[i] > j {
                testree2.insert(v[i]);
            }
        }
    }

    //test
    assert_eq!(testree1.get_hash().unwrap(), testree2.get_hash().unwrap());
    assert!(debug_functions::same_structure(&testree1, &testree2));

    //Test contains_invariance
    for i in 0..v.len() {
        assert_eq!(testree1.contains(v[i]), testree2.contains(v[i]));
    }
}

#[test]
// Bad test
fn multiple_insert_variance() {
    let mut testree1 = MerkleTree::new();
    let mut testree2 = MerkleTree::new();

    let v = rand_bool_vec(5u64, 100);
    // add vertices
    for i in 0..v.len() {
        testree1.insert(i);
        testree2.insert(i);
        if v[i] {
            testree1.insert(i);
        }
    }
    assert_ne!(testree1.get_hash(), testree2.get_hash());
}



fn rand_bool_vec(seed: u64, n: usize) -> Vec<bool> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut v: Vec<bool> = vec![];
    for _ in 0..n {
        v.push(rng.gen());
    }
    v
}


fn rand_int_vec(seed: u64, n: usize, max_value: i32) -> Vec<i32> {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let mut v: Vec<i32> = vec![];
    for _ in 0..n {
        v.push(rng.gen_range(-max_value..max_value+1));
    }
    v
}

/*

fn random_tree(seed1: u64) -> TreeNode {
    let mut testree1 = TreeNode::new();
    let range1: Vec<bool> = rand_bool_vec(seed1, 50);

    for i in 0..range1.len() {
        if range1[i] {
            testree1.insert(i);
        }
    }
    testree1
}



//this test can fail in theory 1/2^100 prob
//#[test]
fn root_hash_uniqueness() {
    for i in 0..5 {
        let testree1 = random_tree(i);
        for j in 0..5 {
            let testree2 = random_tree(j);
            if i == j {
                assert_eq!(testree1.hash, testree2.hash);
            } else {
                assert_ne!(testree1.hash, testree2.hash);
            }
        }
    }
}

 */
