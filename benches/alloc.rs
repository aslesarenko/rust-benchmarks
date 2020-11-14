#![feature(test)]
#[allow(dead_code)]
extern crate test;

use test::Bencher;
use std::hint::black_box;
use typed_arena::Arena;

mod helpers;

const NUM_ITEMS: usize = 10000;
const EXPECTED_SUM: i32 = 49_995_000;

#[bench]
fn loop_alloc_arena(b: &mut Bencher) {
    let mut res: i32 = 0;
    b.iter(|| {
        let values = Arena::new();
        let mut buf = [&0i32; NUM_ITEMS];
        for i in 0..NUM_ITEMS {
            let b = black_box(values.alloc(i as i32));
            buf[i] = b;
        }
        let s = buf.iter().map(|x|*x).sum::<i32>();
        res = s;
        black_box(s);
    });
    assert_eq!(res, EXPECTED_SUM);
}

#[bench]
fn loop_alloc_arena_into_vec(b: &mut Bencher) {
    let mut res: i32 = 0;
    b.iter(|| {
        let values = Arena::with_capacity(NUM_ITEMS);
        let mut buf = Vec::with_capacity(NUM_ITEMS);
        for i in 0..NUM_ITEMS {
            let b: &i32 = black_box(values.alloc(i as i32));
            buf.push(b);
        }
        let s = buf.iter().map(|x|*x).sum::<i32>();
        res = s;
        black_box(s);
    });
    assert_eq!(res, EXPECTED_SUM);
}

#[bench]
fn loop_alloc_boxes(b: &mut Bencher) {
    let mut res: i32 = 0;
    b.iter(|| {
        let mut buf = Vec::with_capacity(NUM_ITEMS);
        for i in 0..NUM_ITEMS {
            let b = black_box(Box::new(i as i32));
            buf.push(b);
        }
        let s = buf.iter().map(|x|**x).sum::<i32>();
        res = s;
        assert_eq!(buf.len(), NUM_ITEMS);
    });
    assert_eq!(res, EXPECTED_SUM);
}

