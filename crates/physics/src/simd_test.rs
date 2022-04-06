// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use std::time::Instant;
//use core_simd::*;
use std::cmp;
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
    let mut pts = h1.generate_random_points(particle_count);

    let mut standard_time = 0;
    let mut simd_time = 0;
    

    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

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
        // this method is almost half the time of seperate loops!
        // so lets keep developing this to make it clean and easy
        println!("iterator test ------------>");
        let start = Instant::now();

        let mut buff = h1.buffer.current.borrow_mut();
        for cell in h1.iter() {
            // for each bucket within the x/y cell....
            let bucket_length = buff.bucket_sz[cell.i as usize] as isize;
            if bucket_length <= 0 {
                continue;
            }


            // update_velocity_from_collisions - part 1
            const cells_to_scan: isize = 3; // need to account for up to 2 * radius

            let iy2_start = if cell.iy < cells_to_scan { 0 } else { cell.iy - cells_to_scan }; //cmp::max(iy - cells_to_scan, 0);
            let iy2_end = cmp::min(cell.fluid_sim.y_size as isize, cell.iy + cells_to_scan);

            let ix2_start = if cell.ix < cells_to_scan { 0 } else { cell.ix - cells_to_scan }; //cmp::max(ix - cells_to_scan, 0);
            let ix2_end = cmp::min(cell.fluid_sim.x_size as isize, cell.ix + cells_to_scan);
            
            //let mut check_icell = cell.ix + (cell.iy * cell.fluid_sim.y_size as isize);
            //check_icell *= cell.fluid_sim.bucket_size as isize;

            for b in cell.icell..(cell.icell + bucket_length) {
                // add_uniform_velocity
                buff.vel[b as usize] += gravity;

                //
                // // update_velocity_from_collisions - part 2
                // get the position of the circle
                let pt = buff.pos[b as usize];
                
                for iy2 in iy2_start..iy2_end {
                    for ix2 in ix2_start..ix2_end {
                        let mut cell2 = ix2 + (iy2 * cell.fluid_sim.y_size as isize);

                        let bucket2_length = buff.bucket_sz[cell2 as usize];
                        cell2 *= cell.fluid_sim.bucket_size as isize;
                        for b2 in 0..bucket2_length {
                            let bucket_cell2 = cell2 + b2 as isize;

                            // don't compare against ourself
                            if b == bucket_cell2 {
                                continue;
                            }

                            // get the position of the circle
                            let pt2 = buff.pos[bucket_cell2 as usize];
                            //let pt_y2 = buff.pos[bucket_cell2+1];


                            // now do a radius check for other circles
                            //print!("  compare against -> pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell2);

                            // compute dist between
                            let a = pt2 - pt;
                            let dist_squared = (a[0] * a[0]) + (a[1] * a[1]);
                            if dist_squared >= dist_squared_max {
                                // no collision
                                //println!(" -> NO collision");
                                continue;
                            }

                            // collision!
                            //println!(" -> collision");

                            // compute and apply velocity to each circle
                            let dist = dist_squared.sqrt();
                            let dist_to_move = dist * 0.5;

                            /*
                            if (dist_to_move < 0.5) {
                                println!("to much intersection")
                            }*/

                            // as the points get closer, the velocity increases
                            // exponentially
                            // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                            let mut vel_mag = 1.0 / dist_to_move;

                            let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);

                            // lose or gain energy in the outgoing velocity
                            let vel = (a * vel_m) * Simd::from_array([cell.fluid_sim.elasticity, cell.fluid_sim.elasticity]);
                            //let vel_y = (a[1] * vel_mag) * self.elasticity;

                            // loose some energy from the incoming velocity
                            //buff.vel[bucket_cell] *= self.collision_energy_loss;
                            //buff.vel[bucket_cell+1] *= self.collision_energy_loss;

                            //buff.vel[bucket_cell2] *= self.collision_energy_loss;
                            //buff.vel[bucket_cell2+1] *= self.collision_energy_loss;

                            buff.vel[b as usize] -= vel;
                            //buff.vel[bucket_cell+1] -= vel_y;

                            //buff.vel[bucket_cell2] += vel_x;
                            //buff.vel[bucket_cell2+1] += vel_y;

                            //println!("done");
                        }
                    }
                }

            }


        }

        let duration = start.elapsed();
        standard_time += duration.as_nanos();
        println!("iterator test - {:?}ns", duration.as_nanos());
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
        let radius2 = 1.0 * scale;
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