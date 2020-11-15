
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr::{ExprTree, Expr};

const NUM_ITEMS: usize = 1000;

fn create_tree() -> ExprTree {
   ExprTree::binary(Expr::Binary(Expr::str_lit("") + Expr::str_lit("")) + Expr::int_lit(10))
}

#[bench]
fn loop_create_tree(b: &mut Bencher) {
    b.iter(||{
        for _ in 0..NUM_ITEMS {
            black_box(create_tree());
        }
    })
}