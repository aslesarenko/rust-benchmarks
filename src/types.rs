use std::ops::{Add, Mul};
use std::iter::Sum;

#[derive(Copy, Clone)]
pub struct Int(pub i32);

#[derive(Copy, Clone)]
pub struct Float(pub f32);

impl<'a> Add<&'a Int> for Int {
    type Output = Int;
    fn add(self, rhs: &'a Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl Mul<Int> for Int {
    type Output = Int;
    fn mul(self, rhs: Int) -> Int {
        Int(self.0 * rhs.0)
    }
}

impl<'a> Sum<&'a Int> for Int {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(Int(0), Add::add)
    }
}

impl<'a> Add<&'a Float> for Float {
    type Output = Float;
    fn add(self, rhs: &'a Float) -> Self::Output {
        Float(self.0 + rhs.0)
    }
}

impl<'a> Sum<&'a Float> for Float {
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(Float(0.0), Add::add)
    }
}

impl Mul<Float> for Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Float {
        Float(rhs.0 * self.0)
    }
}

impl From<usize> for Int {
    fn from(x: usize) -> Self {
        Int(x as i32)
    }
}

impl From<usize> for Float {
    fn from(x: usize) -> Self {
        Float(x as f32)
    }
}
