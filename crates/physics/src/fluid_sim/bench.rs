extern crate test;

#[bench]
fn fluid_sim(b: &mut Bencher) {
    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);
    const radius: f32 = 1.0;
    const dist_squared_max: f32 = (radius + radius) * (radius + radius);

    const GRID_SIZE: usize = 3000;
    const PARTICLE_COUNT: usize = 20000;

    let mut fs = FluidSim::new(GRID_SIZE, GRID_SIZE);
    let particles = fs.generate_random_particles(PARTICLE_COUNT);
    fs.add_particles(&particles);

    b.iter(|| {
        fluid_sim.update(0.001);
    });
}