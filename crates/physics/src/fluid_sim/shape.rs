use crate::fluid_sim::particle::Particle;
use crate::fluid_sim::fluid_sim::Properties;

pub trait Shape {
    fn collide_with(&self, particle: &mut Particle, properties: &Properties);
}