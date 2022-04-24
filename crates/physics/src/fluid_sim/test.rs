use crate::fluid_sim::fluid_sim::FluidSim;
use std::time::Instant;
use core_simd::*;

pub fn test() {
    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

    const GRID_SIZE: usize = 300;
    const PARTICLE_COUNT: usize = 2000;

    let mut fs = FluidSim::new(GRID_SIZE, GRID_SIZE);
    let particles = fs.generate_random_particles(PARTICLE_COUNT);
    fs.add_particles(&particles);

    println!("fluid sim test ------------>");
    let start = Instant::now();

    fs.update(0.001);

    let duration = start.elapsed();
    println!("fluid sim test - {:?}ns", duration.as_nanos());
}