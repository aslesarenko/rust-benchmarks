#![allow(unused_imports)]
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr_tree::{ExprTree, Expr};
use std::borrow::BorrowMut;
use typed_arena::Arena;

const NUM_ITEMS: usize = 1000;

fn create_tree<'a>(t: &'a ExprTree<'a>)  {
    let l = t.str_lit("");
    let r = t.str_lit("");
    let op = t.binary(l, r);
    let i = t.int_lit(10);
    t.set_root(t.binary(op, i));
}

fn create_tree2<'a>(t: &'a ExprTree<'a>)  {
    let i10 = t.int_lit(10);
    let i20 = t.int_lit(20);
    let i30 = t.int_lit(30);
    let i40 = t.int_lit(40);
    let i50 = t.int_lit(50);
    let i60 = t.int_lit(60);
    let expr = t.binary(t.binary(t.binary(t.binary(t.binary(i10, i20), i30), i40), i50), i60);
    t.set_root(expr);
}

/// Expected:
/// osArch: x86_64
/// osName: Mac OS X
/// 127,787 ns/iter (+/- 15,794)
#[bench]
fn loop_create_tree_arena(b: &mut Bencher) {
    b.iter(||{
        let arena = Arena::with_capacity(NUM_ITEMS);
        let t = ExprTree::new(&arena);
        for _ in 0..NUM_ITEMS {
            black_box(create_tree(&t));
        }
    })
}

/// Expected:
/// osArch: x86_64
/// osName: Mac OS X
/// 201,943 ns/iter (+/- 5,523)
#[bench]
fn loop_create_tree_arena2(b: &mut Bencher) {
    b.iter(||{
        let arena = Arena::with_capacity(NUM_ITEMS);
        let t = ExprTree::new(&arena);
        for _ in 0..NUM_ITEMS {
            black_box(create_tree2(&t));
        }
    });
}