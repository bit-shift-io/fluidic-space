// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use std::time::Instant;
use core_simd::*;

use crate::spatial_hash::*;

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

pub fn simd_test() {
    let mut h1 = SpatialHash::new(8, 8, 8); //create_spatial_hash(10, 10, 8);
    let mut h2 = SpatialHash::new(8, 8, 8);
    let mut v0 = h1.generate_random_points(2);


    {/*
        let start = Instant::now();
        simd_add_assign(&mut v0, &mut v1);
        let duration = start.elapsed();
        println!("simd_add_assign - {:?}ns", duration.as_nanos());
    */}

    {/*
        let start = Instant::now();
        add_assign(&mut v0, &mut v1);
        let duration = start.elapsed();
        println!("add_assign - {:?}ns", duration.as_nanos());
    */}

    

    {
        println!("add_points ------------>");
        let start = Instant::now();
        h1.add_points(&v0);
        let duration = start.elapsed();
        println!("add_points - {:?}ns", duration.as_nanos());
    }

    {
        println!("add_points_simd ------------>");
        let start = Instant::now();
        h2.add_points_simd(&v0);
        let duration = start.elapsed();
        println!("add_points_simd - {:?}ns", duration.as_nanos());
    }

    {
        println!("update_velocity_from_collisions ------------>");
        let start = Instant::now();
        h1.update_velocity_from_collisions();
        let duration = start.elapsed();
        println!("update_velocity_from_collisions - {:?}ns", duration.as_nanos());
    }

    {
        println!("update_velocity_from_collisions_simd ------------>");
        let start = Instant::now();
        h2.update_velocity_from_collisions_simd();
        let duration = start.elapsed();
        println!("update_velocity_from_collisions_simd - {:?}ns", duration.as_nanos());
    }

    {
        println!("apply_velocity ------------>");
        let start = Instant::now();
        h1.apply_velocity(1.0);
        let duration = start.elapsed();
        println!("apply_velocity - {:?}ns", duration.as_nanos());
    }

    {
        println!("apply_velocity_simd ------------>");
        let start = Instant::now();
        h1.apply_velocity_simd(1.0);
        let duration = start.elapsed();
        println!("apply_velocity_simd - {:?}ns", duration.as_nanos());
    }

    println!("benchmarking done -----------------------");
}