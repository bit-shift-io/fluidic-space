use crate::fluid_sim_2::particle::Particle;
use crate::fluid_sim_2::fluid_sim_2::Properties;

pub trait Shape {
    fn collide_with(&self, particle: &mut Particle, properties: &Properties);
}