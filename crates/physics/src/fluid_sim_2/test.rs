use crate::fluid_sim_2::fluid_sim_2::FluidSim2;
//use crate::fluid_sim_2::particle::Particle;
use crate::fluid_sim_2::spatial_hash::SpatialHash;
use crate::fluid_sim_2::spatial_hash_iter::SpatialHashIter;
use std::time::Instant;
use core_simd::*;

pub fn test() {
    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

    const GRID_SIZE: usize = 300;
    const PARTICLE_COUNT: usize = 2000;

    let mut fs = FluidSim2::new(GRID_SIZE, GRID_SIZE);
    let particles = fs.generate_random_particles(PARTICLE_COUNT);
    fs.add_particles(&particles);

    println!("fluid sim 2 iterator test ------------>");
    let start = Instant::now();

    let mut cell_it = SpatialHashIter::new(&fs.spatial_hash);
    while cell_it.next() {
        let cell = cell_it.cell();
        for particle_it in cell {
            let particle = *particle_it;

            unsafe {
                // add_uniform_velocity
                (*particle).vel += gravity;
            }

            // make a new region from the current iterator
            // which we get to check each particle in each of those cells for collisions
            let mut col_cell_it = SpatialHashIter::new_region(&cell_it, 3); // need to account for up to 2 * radius
            while col_cell_it.next() {
                let col_cell = col_cell_it.cell();
                for col_particle_it in col_cell {
                    let col_particle = *col_particle_it;
                    //println!("col cell");

                    unsafe {
                        // collision check
                        let pos_delta = (*col_particle).pos - (*particle).pos;
                        let dist_squared = (pos_delta[0] * pos_delta[0]) + (pos_delta[1] * pos_delta[1]);
                        if dist_squared <= 0.0 || dist_squared >= dist_squared_max {
                            // no collision or collision with self
                            //println!(" -> NO collision");
                            continue;
                        }

                        // compute and apply velocity to each circle
                        let dist = dist_squared.sqrt();
                        let dist_to_move = dist * 0.5;

                        // as the points get closer, the velocity increases
                        // exponentially
                        // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                        let mut vel_mag = 1.0 / dist_to_move;

                        let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);

                        // lose or gain energy in the outgoing velocity
                        let vel = (pos_delta * vel_m) * Simd::from_array([fs.elasticity, fs.elasticity]);

                        (*particle).vel -= vel;
                    }
                }
            }
        }
    }


    fs.spatial_hash.clear();

    // the second pass,
    // we move the particles
    let dt = 0.1;
    let dt2: f32x2 = Simd::from_array([dt, dt]);
    for particle in fs.particles.iter_mut() {
        particle.move_reflect(&fs.spatial_hash, dt2);
        fs.spatial_hash.add_particle(particle);
    }

    // simulate render
    for particle in fs.particles.iter_mut() {
        const scale: f32 = 1.0;
        const x_offset: f32 = 1.0;
        const y_offset: f32 = 1.0;
        let x2 = particle.pos[0] * scale + x_offset;
        let y2 = particle.pos[1] * scale + y_offset;
        let radius2 = 1.0 * scale;
    }

    let duration = start.elapsed();
    println!("fluid sim 2 iterator test - {:?}ns", duration.as_nanos());
}