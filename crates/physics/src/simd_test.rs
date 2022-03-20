/* simd.rs
//#![feature(core)]
#![feature(portable_simd)]

use std::simd::f32x4;

pub fn simd_test() {
    // create simd vectors
    let x = f32x4(1.0, 2.0, 3.0, 4.0);
    let y = f32x4(4.0, 3.0, 2.0, 1.0);

    // simd product
    let z = x * y;

    // like any struct, the simd vector can be destructured using `let`
    let f32x4(a, b, c, d) = z;

    println!("{:?}", (a, b, c, d));
}
*/

// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use core_simd::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn simd_test() {
    // create an vector of random points
    let range = Uniform::from(0..100);
    let mut rng = rand::thread_rng();

    // https://stackoverflow.com/questions/48218459/how-do-i-generate-a-vector-of-random-numbers-in-a-range
    let values: Vec<u64> = rand::thread_rng().sample_iter(&range).take(100).collect();

    println!("{:?}", values);

    let a = f32x2::splat(10.0);
    let b = f32x2::from_array([1.0, 2.0]);

    let v: Vec<f32x2> = Vec::new();

    println!("{:?}", a + b);
}