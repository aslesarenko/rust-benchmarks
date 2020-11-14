#![allow(dead_code)]

use benchmarks::types::{Int, Float};
use std::ops::Mul;

#[inline(never)]
pub fn identity<T>(x: T) -> T { x }

#[inline(never)]
pub fn to_float(x: Int) -> Float {
    Float(x.0 as f32)
}

#[inline(always)]
pub fn to_float_inline(x: Int) -> Float {
    Float(x.0 as f32)
}

#[inline(never)]
pub fn to_sqrt(x: i32) -> f32 {
    (x as f32).sqrt()
}

#[inline(always)]
pub fn to_sqrt_inline(x: i32) -> f32 {
    (x as f32).sqrt()
}

#[inline(never)]
pub fn to_sqrt_boxed(x: i32) -> Box<f32> {
    Box::new((x as f32).sqrt())
}

#[inline(never)]
pub fn gen_vec<T>(n: usize) -> Vec<T>
    where T: Mul + From<usize>
{
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(T::from(i * i));
    }
    v
}

