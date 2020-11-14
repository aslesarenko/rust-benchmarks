#![feature(test)]
#[allow(dead_code)]
extern crate test;

use test::Bencher;
use std::hint::black_box;

mod helpers;


const NUM_ITEMS: usize = 1000;

#[bench]
fn loop_sum_int(b: &mut Bencher) {
    b.iter(|| {
        let mut s: i32 = black_box(0);
        for i in 0..NUM_ITEMS {
            let b = black_box(Box::new(i as i32));
            s += *b;
        }
        black_box(s);
    });
}
