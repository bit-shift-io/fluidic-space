use crate::particle::Particle;
use crate::fluid_sim::Properties;

pub trait Shape {
    fn collide_with(&self, particle: &mut Particle, properties: &Properties);
}