#![allow(dead_code)]
use std::ops::Add;

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

pub struct ExprBinary {
    pub attrs: Vec<Attribute>,
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl Add for Expr {
    type Output = ExprBinary;

    fn add(self, rhs: Self) -> Self::Output {
        ExprBinary {
            attrs: vec![],
            left: Box::new(self),
            op: BinOp::Add,
            right: Box::new(rhs),
        }
    }
}

pub enum Expr {
    /// A literal in place of an expression: `1`, `"foo"`.
    Lit(ExprLit),
    /// A binary operation: `a + b`, `a * b`.
    Binary(ExprBinary),
}
impl Expr {
    pub fn str_lit(l: &str) -> Expr {
      Expr::lit(Lit::Str(LitStr(l.into())))
    }
    pub fn int_lit(i: i32) -> Expr {
      Expr::lit(Lit::Int(LitInt(i)))
    }
    pub fn lit(l: Lit) -> Expr {
        Expr::Lit(ExprLit { attrs: vec![], lit: l, })
    }
}

pub struct ExprTree {
    root: Expr
}

impl ExprTree {
    pub fn binary(op: ExprBinary) -> Self {
        ExprTree {
            root: Expr::Binary(op)
        }
    }
}