use crate::fluid_sim_2::particle::Particle;

pub trait Shape {
    fn collide_with(&self, particle: &mut Particle);
}