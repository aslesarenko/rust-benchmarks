#![feature(test)]
#![allow(dead_code)]

extern crate test;

use test::Bencher;
use test::black_box;
use benchmarks::types::{Int, Float};
use benchmarks::helpers::*;

const NUM_ITEMS: usize = 1000;

#[bench]
fn loop_sum_int(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let mut s: i32 = 0;
        for i in 0..NUM_ITEMS {
            s += vec[i].0;
        }
        black_box(s);
    });
}

#[bench]
fn iter_sum_int(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let s = vec.iter().sum::<Int>();
        black_box(s);
    });
}

#[bench]
fn iter_sum_usize(b: &mut Bencher) {
    let vec = gen_vec::<usize>(NUM_ITEMS);
    b.iter(|| {
        let s = vec.iter().sum::<usize>();
        black_box(s);
    });
}

#[bench]
fn iter_sum_float(b: &mut Bencher) {
    let vec = gen_vec::<Float>(NUM_ITEMS);
    b.iter(|| {
        let s = vec.iter().sum::<Float>();
        black_box(s);
    });
}

#[bench]
fn loop_sum_float(b: &mut Bencher) {
    let vec = gen_vec::<Float>(NUM_ITEMS);
    b.iter(|| {
        let mut x: f32 = 0.0;
        for i in 0..NUM_ITEMS {
            x += vec[i].0;
        }
        black_box(x);
    });
}

#[bench]
fn loop_map_float_identity(b: &mut Bencher) {
    let vec = gen_vec::<Float>(NUM_ITEMS);
    b.iter(|| {
        let mut x = Vec::with_capacity(NUM_ITEMS);
        for i in 0..NUM_ITEMS {
            x.push(identity(vec[i]));
        }
        black_box(x);
    });
}

#[bench]
fn iter_map_float_identity(b: &mut Bencher) {
    let vec = gen_vec::<Float>(NUM_ITEMS);
    b.iter(|| {
        let x = vec.iter().map(|&x| identity(x)).collect::<Vec<Float>>();
        black_box(x);
    });
}

#[bench]
fn iter_map_int_to_float(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let x = vec.iter().map(|&x| to_float(x)).collect::<Vec<Float>>();
        black_box(x);
    });
}

#[bench]
fn iter_map_int_to_float_inline(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let x = vec.iter().map(|&x| to_float_inline(x)).collect::<Vec<Float>>();
        black_box(x);
    });
}

#[bench]
fn loop_map_int_to_float_inline(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let mut x = Vec::with_capacity(NUM_ITEMS);
        for i in 0..NUM_ITEMS {
            x.push(to_float_inline(vec[i]));
        }
        black_box(x);
    });
}

#[bench]
fn loop_map_int_to_float(b: &mut Bencher) {
    let vec = gen_vec::<Int>(NUM_ITEMS);
    b.iter(|| {
        let mut x = Vec::with_capacity(NUM_ITEMS);
        for i in 0..NUM_ITEMS {
            x.push(to_float(vec[i]));
        }
        black_box(x);
    });
}


