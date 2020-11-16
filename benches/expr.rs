
use test::Bencher;
use std::hint::black_box;
use benchmarks::expr::{ExprTree, Expr};

const NUM_ITEMS: usize = 1000;

fn create_tree() -> ExprTree {
   ExprTree::new(Expr::str_lit("") + Expr::str_lit("") + Expr::int_lit(10))
}

fn create_tree2() -> ExprTree {
    let expr =
        Expr::int_lit(10) + Expr::int_lit(20) + Expr::int_lit(30) +
        Expr::int_lit(40) + Expr::int_lit(50) + Expr::int_lit(60);
    ExprTree::new(expr)
}

/// Expected:
/// osArch: x86_64
/// osName: Mac OS X
/// 783,153 ns/iter (+/- 36,906)
#[bench]
fn loop_create_tree(b: &mut Bencher) {
    b.iter(||{
        for _ in 0..NUM_ITEMS {
            black_box(create_tree());
        }
    })
}

/// Expected:
/// osArch: x86_64
/// osName: Mac OS X
/// 2,030,305 ns/iter (+/- 39,707)
#[bench]
fn loop_create_tree2(b: &mut Bencher) {
    b.iter(||{
        for _ in 0..NUM_ITEMS {
            black_box(create_tree2());
        }
    })
}