use core_simd::*;
use std::cmp;
use rand::distributions::{Distribution, Uniform};

use crate::fluid_sim_2::spatial_hash::SpatialHash;
use crate::fluid_sim_2::spatial_hash_iter::SpatialHashIter;
use crate::fluid_sim_2::particle::Particle;
pub use crate::fluid_sim_2::test::*;

pub struct FluidSim2 {
    pub spatial_hash: SpatialHash,
    pub particles: Vec<Particle>,
    pub collision_energy_loss: f32, // when colliding, energy loss on velocity
    pub elasticity: f32, // when intersecting what to multiply velocity by. Lower means particles can squish together more
    pub damping: f32, // energy loss. Higher means velocity becomes more like viscous - honey. Lower more like water
    pub gravity: f32x2,
    pub radius: f32,
    pub dist_squared_max: f32,
}

impl FluidSim2 {
    pub fn new(x_size: usize, y_size: usize) -> FluidSim2 {
        let radius: f32 = 1.0;
        FluidSim2 {
            spatial_hash: SpatialHash::new(x_size, y_size),
            collision_energy_loss: 1.0,
            elasticity: 1.0,
            damping: 1.0,
            particles: vec![],
            gravity: Simd::from_array([0.0, 0.3]),
            radius,
            dist_squared_max: (radius + radius) * (radius + radius)
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
        self.spatial_hash.add_particles(&mut self.particles);
    }

    pub fn update(&mut self, dt: f32) {
        let elasticity: f32x2 = Simd::from_array([self.elasticity, self.elasticity]);
        let damping: f32x2 = Simd::from_array([self.damping, self.damping]);

        let mut cell_it = SpatialHashIter::new(&self.spatial_hash);
        while cell_it.next() {
            let cell = cell_it.cell();
            for particle_it in cell {
                let particle = *particle_it;
    
                unsafe {
                    // add_uniform_velocity
                    (*particle).vel += self.gravity;
                }
    
                // make a new region from the current iterator
                // which we get to check each particle in each of those cells for collisions
                let mut col_cell_it = SpatialHashIter::new_region(&cell_it, (self.radius * 2.0) as usize); // need to account for up to 2 * radius
                while col_cell_it.next() {
                    let col_cell = col_cell_it.cell();
                    for col_particle_it in col_cell {
                        let col_particle = *col_particle_it;
                        //println!("col cell");
    
                        unsafe {
                            // collision check
                            let pos_delta = (*col_particle).pos - (*particle).pos;
                            let dist_squared = (pos_delta[0] * pos_delta[0]) + (pos_delta[1] * pos_delta[1]);
                            if dist_squared <= 0.0 || dist_squared >= self.dist_squared_max {
                                // no collision or collision with self
                                //println!(" -> NO collision");
                                continue;
                            }
    
                            // compute and apply velocity to each circle
                            let dist = dist_squared.sqrt();
                            let dist_to_move = dist * 0.5;
    
                            // as the points get closer, the velocity increases
                            // exponentially
                            // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                            let mut vel_mag = 1.0 / dist_to_move;
    
                            let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);
    
                            // lose or gain energy in the outgoing velocity
                            let vel = (pos_delta * vel_m) * elasticity;
    
                            (*particle).vel -= vel;
                        }
                    }
                }
            }
        }
    
        self.spatial_hash.clear();
    
        // the second pass,
        // we move the particles
        let dt2: f32x2 = Simd::from_array([dt, dt]);
        for particle in self.particles.iter_mut() {
            particle.move_reflect(&self.spatial_hash, dt2);
            particle.vel *= damping;
            self.spatial_hash.add_particle(particle);
        }
    }
}