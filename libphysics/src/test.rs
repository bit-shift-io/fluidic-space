use crate::fluid_sim::FluidSim;
use std::time::Instant;

pub fn test() {
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