use crate::FluidSim2;
use crate::fluid_sim_2::spatial_hash_iter::SpatialHashIter;
use std::time::Instant;
use core_simd::*;

pub fn test() {
    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

    const grid_size: usize = 3;
    const particle_count: usize = 2;

    let mut fs = FluidSim2::new(grid_size, grid_size);
    let particles = fs.generate_random_particles(particle_count);
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
        }
    }

    let duration = start.elapsed();
    println!("fluid sim 2 iterator test - {:?}ns", duration.as_nanos());
}