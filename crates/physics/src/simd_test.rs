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
    sz: Vec<usize>,
    x_size: usize,
    y_size: usize,
    bucket_size: usize
}

impl SpatialHash {
    fn new(x_size: usize, y_size: usize, bucket_size: usize) -> SpatialHash {
        let total_size = x_size * y_size * bucket_size;
        let xy_size = x_size * y_size;
        SpatialHash {
            hash: vec![0.0; total_size],
            sz: vec![0; xy_size],
            x_size,
            y_size,
            bucket_size
        }
    }

    // simd accelerated
    fn add_points_simd(&mut self, pts: &Vec<f32>) {
        // assert!(pts.len() multiple of 4)

        let size = pts.len() as isize;
        let chunks = size / 4;
    
        // treat f32 vector as an simd f32x4 vector
        let ptr = pts.as_ptr() as *const f32x4;

        //let simd_p_x = p_x as *mut f32x4;
    
        let size_mult: u32x4 = Simd::from_array([1, self.y_size as u32, 1, self.y_size as u32]);

        // sum excess elements that don't fit in the simd vector
        for i in 0..chunks {
            // dereferencing a raw pointer requires an unsafe block
            unsafe {
                // offset by i elements
                let fv: f32x4 = *ptr.offset(i);
                let iv: u32x4 = fv.cast::<u32>(); //i32x4::from(fv); // fv;
                let iv_size = iv * size_mult;

                // can we simd this + operation?
                // is there a conversion penalty from u32 to usize?
                // if so, can we somehow avoid it by making the vector u32 referencable? 
                let cell1 = (iv_size[0] + iv_size[1]) as usize;
                let cell2 = (iv_size[2] + iv_size[3]) as usize;

                //assert!(self.sz[cell1] < self.bucket_size);
                self.sz[cell1] += 1;

                //assert!(self.sz[cell2] < self.bucket_size);
                self.sz[cell2] += 1;
            }
        }
    }

    // bog standard
    fn add_points(&mut self, pts: &Vec<f32>) {
        for i in (0..pts.len()).step_by(2) {
            let x = pts[i];
            let y = pts[i+1];

            //assert!(x >= 0.0);
            //assert!(y >= 0.0);

            let ix: usize = x as usize;
            let iy: usize = y as usize;

            //assert!(ix < self.x_size);
            //assert!(iy < self.y_size);

            let cell = ix + (iy * self.y_size);

            //assert!(self.sz[cell] < self.bucket_size);
            self.sz[cell] += 1;
        }
    }
}


pub fn simd_test() {
    let mut h = SpatialHash::new(8, 8, 8); //create_spatial_hash(10, 10, 8);

    // create an vector of random points
    let mut v0: Vec<f32> = Vec::new();
    let mut v1: Vec<f32> = Vec::new();
    let range = Uniform::from(0.0..8.0);

    let mut rng = rand::thread_rng();
    for n in 1..8 {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);

        v0.push(x);
        v0.push(y);
    }

    for n in 1..8 {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);

        v1.push(x);
        v1.push(y);
    }

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