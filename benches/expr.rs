
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr::{Prog, Expr, LitStr};

const NUM_ITEMS: usize = 10000;

fn create_prog() -> Prog {
   Prog::binary(Expr::strLit("") + Expr::strLit(""))
}

#[bench]
fn loop_create_prog(b: &mut Bencher) {
    b.iter(||{
        for i in 0..NUM_ITEMS {
            black_box(create_prog());
        }
    })
}