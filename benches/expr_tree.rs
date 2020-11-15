#![allow(unused_imports)]
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr_tree::{ExprTree, Expr};
use std::borrow::BorrowMut;
use typed_arena::Arena;

const NUM_ITEMS: usize = 10000;

fn create_tree<'a>(t: &'a ExprTree<'a>)  {
    let l = t.str_lit("1");
    let tup = t.tuple(vec![l]);
    t.set_root(tup);
}

#[bench]
fn loop_create_prog(b: &mut Bencher) {
    b.iter(||{
        for _ in 0..NUM_ITEMS {
            let arena = Arena::new();
            let mut t = ExprTree::new(arena);
            create_tree(&t);
            // let mut t = ExprTree::new(|arena| arena.alloc(Expr::int_lit(10)));
            // black_box(t);
        }
    })
}