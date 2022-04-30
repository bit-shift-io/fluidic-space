use core_simd::*;
use rand::distributions::{Distribution, Uniform};

use crate::rect::Rect;
use crate::shape::Shape;
use crate::spatial_hash::SpatialHash;
use crate::spatial_hash_iter::SpatialHashIter;
use crate::particle::Particle;
use crate::vector_2::*;

pub struct Properties {
    pub collision_energy_loss: f32, // when colliding, energy loss on velocity
    pub elasticity: f32, // when intersecting what to multiply velocity by. Lower means particles can squish together more
    pub damping: f32, // energy loss. Higher means velocity becomes more like viscous - honey. Lower more like water
    pub collision_damping: f32, // energy loss during collisions
    pub gravity: f32x2,
    pub radius: f32,
    pub dist_squared_max: f32,
}

pub struct FluidSim {
    pub spatial_hash: SpatialHash,
    pub particles: Vec<Particle>,
    pub properties: Properties,
    //pub shapes: Vec<Box<dyn Shape>>,
    pub rects: Vec<Rect>,
}

impl FluidSim {
    pub fn new(x_size: usize, y_size: usize) -> FluidSim {
        let radius: f32 = 1.0;
        FluidSim {
            spatial_hash: SpatialHash::new(x_size, y_size),
            properties: Properties {
                collision_energy_loss: 1.0,
                elasticity: 1.0,
                damping: 1.0,
                collision_damping: 1.0,
                radius,
                dist_squared_max: (radius + radius) * (radius + radius),
                gravity: Simd::from_array([0.0, 0.3]),
            },
            particles: vec![],
            //shapes: vec![],
            rects: vec![]
        }
    }

    pub fn generate_random_particles(&self, count: usize) -> Vec<Particle> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = Vec::new();

        for _b in 0..count {
            let pt_x = range.sample(&mut rng) * (self.spatial_hash.x_size as f32);
            let pt_y = range.sample(&mut rng) * (self.spatial_hash.y_size as f32);
            particles.push(Particle::new(Simd::from_array([pt_x, pt_y])));
            //println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
        }

        return particles;
    }

    pub fn add_particles(&mut self, particles: &Vec<Particle>) {
        for particle in particles {
            self.particles.push(particle.clone());
        }
        self.spatial_hash_particles();
    }

    pub fn spatial_hash_particles(&mut self) {
        self.spatial_hash.clear();
        self.spatial_hash.add_particles(&mut self.particles);
    }

    pub fn update(&mut self, dt: f32) {
        let damping: f32x2 = vec2_from_single(self.properties.damping);
        let dt2: f32x2 = vec2_from_single(dt);

        // compute particle contacts
        let mut cell_it = SpatialHashIter::new(&self.spatial_hash);
        while cell_it.next() {
            let cell = cell_it.cell();
            for particle_it in cell {
                let particle = *particle_it;
    
                unsafe {
                    (*particle).check_particle_collisions(&cell_it, &self.properties, dt2);
                }
            }
        }
    
        self.spatial_hash.clear();
    
        // propogate & update velocity
        for particle in self.particles.iter_mut() {
            particle.update_velocity(&self.spatial_hash, &self.properties, dt2);
        }

        // we move the particles
        for particle in self.particles.iter_mut() {
            // collision detection with any rects (TODO: spatial hashing)
            for rect in self.rects.iter() {
                rect.collide_with(particle, &self.properties);
            }

            particle.move_pos(&self.spatial_hash, &self.properties, dt2);

            self.spatial_hash.add_particle(particle);
        }
    }
}