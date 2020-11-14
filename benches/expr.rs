
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr::{Prog, Expr};

const NUM_ITEMS: usize = 10000;

fn create_prog() -> Prog {
   Prog::binary(Expr::str_lit("") + Expr::str_lit(""))
}

#[bench]
fn loop_create_prog(b: &mut Bencher) {
    b.iter(||{
        for _ in 0..NUM_ITEMS {
            black_box(create_prog());
        }
    })
}