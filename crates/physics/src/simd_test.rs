// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use std::time::Instant;
use core_simd::*;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

// element-wise addition
fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    //assert_equal_len!(xs, ys);

    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x += *y;
    }
}

// simd accelerated addition
fn simd_add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    //assert_equal_len!(xs, ys);

    let size = xs.len() as isize;
    let chunks = size / 4;

    // pointer to the start of the vector data
    let p_x: *mut f32 = xs.as_mut_ptr();
    let p_y: *const f32 = ys.as_ptr();

    // sum excess elements that don't fit in the simd vector
    for i in (4 * chunks)..size {
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            // offset by i elements
            *p_x.offset(i) += *p_y.offset(i);
        }
    }

    // treat f32 vector as an simd f32x4 vector
    let simd_p_x = p_x as *mut f32x4;
    let simd_p_y = p_y as *const f32x4;

    // sum "simd vector"
    for i in 0..chunks {
        unsafe {
            *simd_p_x.offset(i) += *simd_p_y.offset(i);
        }
    }
}

struct SpatialHash {
    hash: Vec<f32>,
    sz: Vec<u32>
}

impl SpatialHash {
    fn new(x_size: usize, y_size: usize, bucket_size: usize) -> SpatialHash {
        let total_size = x_size * y_size * bucket_size;
        let xy_size = x_size * y_size;
        SpatialHash {
            hash: vec![0.0; total_size],
            sz: vec![0; xy_size],
        }
    }

    // simd accelerated
    fn add_points_simd(&self, pts: &Vec<f32>) {
        println!(".");
    }

    // bog standard
    fn add_points(&self, pts: &Vec<f32>) {
        println!(".");
    }
}

fn create_spatial_hash(x_size: usize, y_size: usize, bucket_size: usize) -> SpatialHash {
    let total_size = x_size * y_size * bucket_size;
    let xy_size = x_size * y_size;
    SpatialHash {
        hash: vec![0.0; total_size],
        sz: vec![0; xy_size],
    }
    /*
    let mut h: Vec<Vec<f32>> = vec![0; total_size];
    for n in 1..total_size {
        let b: Vec<f32> = vec![0; bucket_size];
        h[n] = b;
    } */
    //return h;
}

pub fn simd_test() {
    let h = SpatialHash::new(10, 10, 8); //create_spatial_hash(10, 10, 8);

    // create an vector of random points
    let mut v0: Vec<f32> = Vec::new();
    let mut v1: Vec<f32> = Vec::new();
    let range = Uniform::from(0.0..100.0);
    let mut rng = rand::thread_rng();
    for n in 1..100 {
        let x = 1.0; //range.sample(&mut rng);
        let y = 1.0; //range.sample(&mut rng);

        v0.push(x);
        v0.push(y);
    }

    for n in 1..100 {
        let x = 10.0; //range.sample(&mut rng);
        let y = 10.0; //range.sample(&mut rng);

        v1.push(x);
        v1.push(y);
    }

    {
        let start = Instant::now();
        simd_add_assign(&mut v0, &mut v1);
        let duration = start.elapsed();
        println!("simd_add_assign - {:?}ns", duration.as_nanos());
    }

    {
        let start = Instant::now();
        add_assign(&mut v0, &mut v1);
        let duration = start.elapsed();
        println!("add_assign - {:?}ns", duration.as_nanos());
    }

    {
        let start = Instant::now();
        h.add_points(&v0);
        let duration = start.elapsed();
        println!("add_points - {:?}ns", duration.as_nanos());
    }

    {
        let start = Instant::now();
        h.add_points_simd(&v0);
        let duration = start.elapsed();
        println!("add_points_simd - {:?}ns", duration.as_nanos());
    }

    println!("done");
}