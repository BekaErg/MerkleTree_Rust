use merkle::*;
use rand::prelude::*;

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};



fn build_tree(n: i32) -> MerkleTree {
    let mut testree = MerkleTree::new();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    // add vertices
    let compression = 3;
    for _i in 0..n {
        let v = rng.gen_range(-n/compression..n/compression);
        testree.insert(v);
    }
    testree
}

fn contains(testree: &mut MerkleTree, n: i32) {

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(2);
    for _i in 0..n {
        let v = rng.gen_range(-n..n);
        testree.contains(v);
    }
}

#[inline]
fn bench_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("group1");
    group.significance_level(0.3).sample_size(10);
    group.bench_function("bench_insert", |b| b.iter(|| build_tree (black_box(1000))));
    group.finish();
}


fn bench_contains(c: &mut Criterion) {
    let n = 10000;
    let mut benchtree = build_tree(n);
    let mut group = c.benchmark_group("group2");
    group.significance_level(0.3).sample_size(10);
    group.bench_function("bench_contains", |b| b.iter(|| contains(& mut benchtree, n)));
    group.finish();
}

criterion_group!(benches, bench_insert, bench_contains);
criterion_main!(benches);




/*

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





fn random_only(n: i32){
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
    let compression = 3;
    for _i in 0..n {
        let v = rng.gen_range(-n/compression..n/compression);
    }
}
fn random(c: &mut Criterion) {
    //let v = rand_int_vec(1u64, 100, 10);
    c.bench_function("random_only", |b| b.iter(|| random_only (10000000)));
}

fn build_tree(v: &Vec<i32>) -> MerkleTree {
    let mut testree = MerkleTree::new();

    // add vertices
    for i in 0..v.len() {
        //assert!(!testree.contains(i));
        for _ in 0..v[i] {
            testree.insert(i);
        }
    }
    testree
}

fn contains(testree: &mut MerkleTree, n: usize) {
    //check
    for i in 0..n {
        testree.contains(i);
    }
}
*/