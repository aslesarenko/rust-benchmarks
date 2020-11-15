#![allow(dead_code)]
#![allow(unused_imports)]
use std::ops::Add;
use typed_arena::Arena;
use std::borrow::Borrow;
use std::cell::Cell;

pub struct Attribute {
    pub name: String,
}

pub struct LitStr(String);
pub struct LitInt(i32);

pub enum Lit {
    /// A UTF-8 string literal: `"foo"`.
    Str(LitStr),
    /// An integer literal: `1` or `1u16`.
    Int(LitInt),
}

pub struct ExprLit {
    pub attrs: Vec<Attribute>,
    pub lit: Lit,
}

pub enum BinOp {
    /// The `+` operator (addition)
    Add,
    /// The `*` operator (multiplication)
    Mul,
}

pub struct ExprBinary<'a> {
    pub attrs: Vec<Attribute>,
    pub left: &'a Expr<'a>,
    pub op: BinOp,
    pub right: &'a Expr<'a>,
}

pub struct ExprTuple<'a> {
    pub elems: Vec<&'a Expr<'a>>,
}

pub enum Expr<'a> {
    /// A literal in place of an expression: `1`, `"foo"`.
    Lit(ExprLit),
    /// A binary operation: `a + b`, `a * b`.
    Binary(ExprBinary<'a>),
    /// A tuple expression: `(a, b, c, d)`.
    Tuple(ExprTuple<'a>),
}
impl<'a> Expr<'a> {
    pub fn str_lit(l: &str) -> Expr<'a> {
      Expr::lit(Lit::Str(LitStr(l.into())))
    }
    pub fn int_lit(i: i32) -> Expr<'a> {
      Expr::lit(Lit::Int(LitInt(i)))
    }
    pub fn lit(l: Lit) -> Expr<'a> {
        Expr::Lit(ExprLit { attrs: vec![], lit: l, })
    }
    pub fn tuple(elems: Vec<&'a Expr<'a>>) -> Expr<'a> {
        Expr::Tuple(ExprTuple { elems })
    }
}
impl<'a> Add for &'a Expr<'a> {
    type Output = ExprBinary<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        ExprBinary {
            attrs: vec![],
            left: self,
            op: BinOp::Add,
            right: rhs,
        }
    }
}


pub struct ExprTree<'a> {
    expr_arena: Arena<Expr<'a>>,
    root: Cell<Option<&'a Expr<'a>>>
}

impl<'a> ExprTree<'a> {
    pub fn new(expr_arena: Arena<Expr<'a>>) -> Self {
        ExprTree::<'a> {
            expr_arena,
            root: Cell::new(None)
        }
    }

    // pub fn build<F>(f: F) -> Self  where F: FnOnce(&'a Self) -> &'a Expr<'a> {
    //     let t = ExprTree::new();
    //     let root = f(&t);
    //     t.root.set(Some(root));
    //     t
    // }

    pub fn str_lit(&'a self, s: &str) -> &'a Expr<'a> {
        self.expr_arena.alloc(Expr::str_lit(s))
    }
    pub fn tuple(&'a self, elems: Vec<&'a Expr<'a>>) -> &'a Expr<'a> {
        self.expr_arena.alloc(Expr::<'a>::tuple(elems))
    }
    pub fn set_root(&self, r: &'a Expr<'a>) {
        self.root.set(Some(r));
    }}