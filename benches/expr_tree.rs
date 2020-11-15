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

#[bench]
fn loop_create_tree_arena(b: &mut Bencher) {
    b.iter(||{
        let arena = Arena::new();
        let t = ExprTree::new(&arena);
        for _ in 0..NUM_ITEMS {
            black_box(create_tree(&t));
            // let mut t = ExprTree::new(|arena| arena.alloc(Expr::int_lit(10)));
            // black_box(t);
        }
    })
}