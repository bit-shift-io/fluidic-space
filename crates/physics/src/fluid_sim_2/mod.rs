use core_simd::*;
use std::cmp;
use rand::distributions::{Distribution, Uniform};

use crate::fluid_sim_2::spatial_hash::SpatialHash;
use crate::fluid_sim_2::spatial_hash_iter::SpatialHashIter;
use crate::fluid_sim_2::particle::Particle;
pub use crate::fluid_sim_2::test::*;

mod spatial_hash;
mod spatial_hash_iter;
mod particle;
mod test;


pub struct FluidSim2 {
    pub spatial_hash: SpatialHash,
    pub particles: Vec<Particle>,
    pub collision_energy_loss: f32, // when colliding, energy loss on velocity
    pub elasticity: f32, // when intersecting what to multiply velocity by. Lower means particles can squish together more
    pub damping: f32, // energy loss. Higher means velocity becomes more like viscous - honey. Lower more like water
}

impl FluidSim2 {
    pub fn new(x_size: usize, y_size: usize) -> FluidSim2 {
        FluidSim2 {
            spatial_hash: SpatialHash::new(x_size, y_size),
            collision_energy_loss: 1.0,
            elasticity: 1.0,
            damping: 1.0,
            particles: vec![]
        }
    }

    pub fn generate_random_particles(&self, count: usize) -> Vec<Particle> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = Vec::new();

        for b in 0..count {
            let pt_x = range.sample(&mut rng) * (self.spatial_hash.x_size as f32);
            let pt_y = range.sample(&mut rng) * (self.spatial_hash.y_size as f32);
            particles.push(Particle::new(Simd::from_array([pt_x, pt_y])));
            //println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
        }

        return particles;
    }

    pub fn add_particles(&mut self, particles: &Vec<Particle>) {
        for particle in particles {
            self.particles.push(*particle);
        }
        self.spatial_hash_particles();
    }

    pub fn spatial_hash_particles(&mut self) {
        self.spatial_hash.clear();
        self.spatial_hash.add(&mut self.particles);
    }
}