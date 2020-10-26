#![feature(test)]

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;
    use std::hint::black_box;
    use std::ops::{Mul, Add};
    use std::iter::Sum;

    #[inline(never)]
    fn identity<T>(x: T) -> T { x }

    #[inline(never)]
    fn to_float(x: Int) -> Float {
        Float(x.0 as f32)
    }

    #[inline(always)]
    fn to_float_inline(x: Int) -> Float {
        Float(x.0 as f32)
    }

    #[inline(never)]
    fn to_sqrt(x: i32) -> f32 {
        (x as f32).sqrt()
    }

    #[inline(always)]
    fn to_sqrt_inline(x: i32) -> f32 {
        (x as f32).sqrt()
    }

    #[inline(never)]
    fn to_sqrt_boxed(x: i32) -> Box<f32> {
        Box::new((x as f32).sqrt())
    }

    const NUM_ITEMS: usize = 1000;

    #[inline(never)]
    fn gen_vec<T>(n: usize) -> Vec<T>
        where T: Mul + From<usize>
    {
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(T::from(i * i));
        }
        v
    }

    #[derive(Copy, Clone)]
    struct Int(i32);

    #[derive(Copy, Clone)]
    struct Float(f32);

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

    #[bench]
    fn loop_sum_int(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let mut s: i32 = 0;
            for i in 0..NUM_ITEMS {
                s += vec[i].0;
            }
            black_box(s);
        });
    }

    #[bench]
    fn iter_sum_int(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let s = vec.iter().sum::<Int>();
            black_box(s);
        });
    }

    #[bench]
    fn iter_sum_usize(b: &mut Bencher) {
        let vec = gen_vec::<usize>(NUM_ITEMS);
        b.iter(|| {
            let s = vec.iter().sum::<usize>();
            black_box(s);
        });
    }

    #[bench]
    fn iter_sum_float(b: &mut Bencher) {
        let vec = gen_vec::<Float>(NUM_ITEMS);
        b.iter(|| {
            let s = vec.iter().sum::<Float>();
            black_box(s);
        });
    }

    #[bench]
    fn loop_sum_float(b: &mut Bencher) {
        let vec = gen_vec::<Float>(NUM_ITEMS);
        b.iter(|| {
            let mut x: f32 = 0.0;
            for i in 0..NUM_ITEMS {
                x += vec[i].0;
            }
            black_box(x);
        });
    }

    #[bench]
    fn loop_map_float_identity(b: &mut Bencher) {
        let vec = gen_vec::<Float>(NUM_ITEMS);
        b.iter(|| {
            let mut x = Vec::with_capacity(NUM_ITEMS);
            for i in 0..NUM_ITEMS {
                x.push(identity(vec[i]));
            }
            black_box(x);
        });
    }

    #[bench]
    fn iter_map_float_identity(b: &mut Bencher) {
        let vec = gen_vec::<Float>(NUM_ITEMS);
        b.iter(|| {
            let x = vec.iter().map(|&x|identity(x)).collect::<Vec<Float>>();
            black_box(x);
        });
    }

    #[bench]
    fn iter_map_int_to_float(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let x = vec.iter().map(|&x|to_float(x)).collect::<Vec<Float>>();
            black_box(x);
        });
    }

    #[bench]
    fn iter_map_int_to_float_inline(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let x = vec.iter().map(|&x|to_float_inline(x)).collect::<Vec<Float>>();
            black_box(x);
        });
    }

    #[bench]
    fn loop_map_int_to_float_inline(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let mut x = Vec::with_capacity(NUM_ITEMS);
            for i in 0..NUM_ITEMS {
                x.push(to_float_inline(vec[i]));
            }
            black_box(x);
        });
    }

    #[bench]
    fn loop_map_int_to_float(b: &mut Bencher) {
        let vec = gen_vec::<Int>(NUM_ITEMS);
        b.iter(|| {
            let mut x = Vec::with_capacity(NUM_ITEMS);
            for i in 0..NUM_ITEMS {
                x.push(to_float(vec[i]));
            }
            black_box(x);
        });
    }

}

