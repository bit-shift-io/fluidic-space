// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use std::time::Instant;
//use core_simd::*;

use crate::fluid_sim::*;
use core_simd::*;

/*
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
*/

pub fn simd_test() {
    const grid_size: usize = 300;
    const particle_count: usize = 2000;
    const max_particles_per_cell: usize = 2;

    let mut h1 = FluidSim::new(grid_size, grid_size, max_particles_per_cell * 2); //create_fluid_sim(10, 10, 8);
    let mut h2 = FluidSim::new(grid_size, grid_size, max_particles_per_cell * 2);
    let mut pts = h1.generate_random_points(2);

    let mut standard_time = 0;
    let mut simd_time = 0;
    
    println!("benchmarking start ----------------------->");

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
        h1.add_points(&pts);
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("add_points - {:?}ns", duration.as_nanos());
    }

    {
        println!("add_points_simd ------------>");
        let start = Instant::now();
        h2.add_points_simd(&pts);
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("add_points_simd - {:?}ns", duration.as_nanos());
    }

    {
        println!("update_velocity_from_collisions ------------>");
        let start = Instant::now();
        h1.update_velocity_from_collisions();
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("update_velocity_from_collisions - {:?}ns", duration.as_nanos());
    }

    {
        println!("update_velocity_from_collisions_simd ------------>");
        let start = Instant::now();
        h2.update_velocity_from_collisions_simd();
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("update_velocity_from_collisions_simd - {:?}ns", duration.as_nanos());
    }

    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);

    {
        println!("add_uniform_velocity ------------>");
        let start = Instant::now();
        h1.add_uniform_velocity(gravity); // some gravity
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("add_uniform_velocity - {:?}ns", duration.as_nanos());
    }

    {
        println!("add_uniform_velocity_simd ------------>");
        let start = Instant::now();
        h2.add_uniform_velocity_simd(gravity); // some gravity
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("add_uniform_velocity_simd - {:?}ns", duration.as_nanos());
    }

    {
        println!("apply_velocity ------------>");
        let start = Instant::now();
        h1.apply_velocity(1.0);
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("apply_velocity - {:?}ns", duration.as_nanos());
    }

    {
        println!("apply_velocity_simd ------------>");
        let start = Instant::now();
        h2.apply_velocity_simd(1.0);
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("apply_velocity_simd - {:?}ns", duration.as_nanos());
    }

    {
        println!("swap ------------>");
        let start = Instant::now();
        h1.swap();
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("swap - {:?}ns", duration.as_nanos());
    }

    {
        println!("swap ------------>");
        let start = Instant::now();
        h2.swap();
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("swap - {:?}ns", duration.as_nanos());
    }

    {
        println!("clear_next ------------>");
        let start = Instant::now();
        h1.clear_next(false);
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("clear_next - {:?}ns", duration.as_nanos());
    }

    {
        println!("clear_next_simd ------------>");
        let start = Instant::now();
        h2.clear_next_simd(false);
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("clear_next_simd - {:?}ns", duration.as_nanos());
    }

    // THIS IS HORRIBLY SLOW! rethink how we do this
    // simulate rendering of particles
    let render_c = #[inline(always)] |pt: f32x2| {
        const scale: f32 = 1.0;
        const x_offset: f32 = 1.0;
        const y_offset: f32 = 1.0;
        let x2 = pt[0] * scale + x_offset;
        let y2 = pt[1] * scale + y_offset;
        let radius = 1.0 * scale;
        // draw
    };

    {
        println!("for_each_pos ------------>");
        let start = Instant::now();
        h1.for_each_pos(render_c);
        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("for_each_pos - {:?}ns", duration.as_nanos());
    }

    {
        println!("for_each_pos_simd ------------>");
        let start = Instant::now();
        h2.for_each_pos_simd(render_c);
        let duration = start.elapsed();
        simd_time += duration.as_nanos();
        println!("for_each_pos_simd - {:?}ns", duration.as_nanos());
    }

    /*
        To render at 30fps, we have a frame time of 33.333ms
    */
    println!("-----------------------");
    let standard_time_ms = standard_time / 1000000;
    let simd_time_ms = simd_time / 1000000;
    println!("standard total  - {:?}ns     - {:?}ms", standard_time, standard_time_ms);
    println!("simd total      - {:?}ns     - {:?}ms", simd_time, simd_time_ms);
    println!("benchmarking done -----------------------");
}