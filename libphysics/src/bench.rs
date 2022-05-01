extern crate test;

#[bench]
fn fluid_sim(b: &mut Bencher) {
    const GRID_SIZE: usize = 3000;
    const PARTICLE_COUNT: usize = 20000;

    let mut fs = FluidSim::new(GRID_SIZE, GRID_SIZE);
    let particles = fs.generate_random_particles(PARTICLE_COUNT);
    fs.add_particles(&particles);

    b.iter(|| {
        fluid_sim.update(0.001);
    });
}