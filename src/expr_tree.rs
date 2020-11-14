#![allow(dead_code)]
use std::ops::Add;
use typed_arena::Arena;

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

pub enum Expr<'a> {
    /// A literal in place of an expression: `1`, `"foo"`.
    Lit(ExprLit),
    /// A binary operation: `a + b`, `a * b`.
    Binary(ExprBinary<'a>),
}
impl<'a> Expr<'a> {
    pub fn str_lit(l: &str) -> Expr {
      Expr::lit(Lit::Str(LitStr(l.into())))
    }
    pub fn int_lit(i: i32) -> Expr<'a> {
      Expr::lit(Lit::Int(LitInt(i)))
    }
    pub fn lit(l: Lit) -> Expr<'a> {
        Expr::Lit(ExprLit { attrs: vec![], lit: l, })
    }
}

pub struct ExprTree<'a> {
    expr_arena: Arena<Expr<'a>>,
    root: Expr<'a>
}

impl<'a> ExprTree<'a> {
    pub fn binary(op: ExprBinary<'a>) -> Self {
        ExprTree {
            expr_arena: Arena::new(),
            root: Expr::Binary(op)
        }
    }
}