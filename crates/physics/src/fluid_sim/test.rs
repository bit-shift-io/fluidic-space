// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/simd.html

use std::time::Instant;
//use core_simd::*;
use std::cmp;
use crate::fluid_sim::*;
//use core_simd::*;

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

pub fn test() {
    const GRID_SIZE: usize = 300;
    const PARTICLE_COUNT: usize = 2000;
    const max_particles_per_pIt: usize = 2;

    let mut f = FluidSim::new(GRID_SIZE, GRID_SIZE, max_particles_per_pIt); //create_fluid_sim(10, 10, 8);
    let mut pts = f.generate_random_points(PARTICLE_COUNT);
    f.add_points(&pts);

    let mut standard_time = 0;
    let mut simd_time = 0;
    

    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

    println!("benchmarking start ----------------------->");

    {
        // this method is almost half the time of seperate loops!
        // so lets keep developing this to make it clean and easy
        println!("iterator test ------------>");
        let start = Instant::now();

        //let mut buff = f.buffer.current.borrow_mut();
        let mut pIt = f.iter();
        //for mut pIt in f.iter() {
        while pIt.next() {

            // add_uniform_velocity
            //pIt.vel() += gravity;
            pIt.buff.vel[pIt.ibuff] += gravity;

            let pt = pIt.buff.pos[pIt.ibuff]; //pIt.pos();

            /*
        //for pIt in f.iter() {
            // for each bucket within the x/y pIt....
            let bucket_length = pIt.buff.bucket_sz[pIt.i as usize] as isize;
            if bucket_length <= 0 {
                continue;
            }*/
            
            // update_velocity_from_collisions - part 1
            const pIts_to_scan: isize = 3; // need to account for up to 2 * radius

            let iy2_start = if pIt.iy < pIts_to_scan { 0 } else { pIt.iy - pIts_to_scan }; //cmp::max(iy - pIts_to_scan, 0);
            let iy2_end = cmp::min(pIt.fluid_sim.y_size as isize, pIt.iy + pIts_to_scan);

            let ix2_start = if pIt.ix < pIts_to_scan { 0 } else { pIt.ix - pIts_to_scan }; //cmp::max(ix - pIts_to_scan, 0);
            let ix2_end = cmp::min(pIt.fluid_sim.x_size as isize, pIt.ix + pIts_to_scan);
            
            //let mut check_ipIt = pIt.ix + (pIt.iy * pIt.fluid_sim.y_size as isize);
            //check_ipIt *= pIt.fluid_sim.bucket_size as isize;

            //for b in pIt.ipIt..(pIt.ipIt + pIt.bucket_length) {
            {
                
                //pIt.buff.vel[b as usize] += gravity;
                //pIt.buff.vel[pIt.ibuff] += gravity;
                

                //
                // // update_velocity_from_collisions - part 2
                // get the position of the circle
                //let pt = pIt.buff.pos[b as usize];
                
                for iy2 in iy2_start..iy2_end {
                    for ix2 in ix2_start..ix2_end {
                        let mut pIt2 = ix2 + (iy2 * pIt.fluid_sim.y_size as isize);

                        let bucket2_length = pIt.buff.bucket_sz[pIt2 as usize];
                        pIt2 *= pIt.fluid_sim.bucket_size as isize;
                        for b2 in 0..bucket2_length {
                            let bucket_pIt2 = pIt2 + b2 as isize;

                            // don't compare against ourself
                            if pIt.ibuff == bucket_pIt2 as usize {
                                continue;
                            }

                            // get the position of the circle
                            let pt2 = pIt.buff.pos[bucket_pIt2 as usize];
                            //let pt_y2 = buff.pos[bucket_pIt2+1];


                            // now do a radius check for other circles
                            //print!("  compare against -> pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_pIt2);

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
                            let vel = (a * vel_m) * Simd::from_array([pIt.fluid_sim.elasticity, pIt.fluid_sim.elasticity]);
                            //let vel_y = (a[1] * vel_mag) * self.elasticity;

                            // loose some energy from the incoming velocity
                            //buff.vel[bucket_pIt] *= self.collision_energy_loss;
                            //buff.vel[bucket_pIt+1] *= self.collision_energy_loss;

                            //buff.vel[bucket_pIt2] *= self.collision_energy_loss;
                            //buff.vel[bucket_pIt2+1] *= self.collision_energy_loss;

                            //pIt.vel() -= vel;
                            pIt.buff.vel[pIt.ibuff] -= vel;

                            //pIt.buff.vel[b as usize] -= vel;
                            //buff.vel[bucket_pIt+1] -= vel_y;

                            //buff.vel[bucket_pIt2] += vel_x;
                            //buff.vel[bucket_pIt2+1] += vel_y;

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