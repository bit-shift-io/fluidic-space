use core_simd::*;
use crate::fluid_sim::spatial_hash::SpatialHash;
use crate::fluid_sim::fluid_sim::Properties;
use crate::*;

#[derive(Clone)]
pub struct Contact {
    //pub pos: f32x2,
    pub normal: f32x2,
    pub depth: f32,
    //pub particle: &Particle
}

#[derive(Clone)]
pub struct Particle {
    pub pos: f32x2,
    pub vel: f32x2,
    pub contacts: Vec<Contact>,
}

impl Particle {
    pub fn new(pos: f32x2) -> Particle {
        Particle{
            pos,
            vel: Simd::from_array([0.0, 0.0]),
            contacts: Vec::new()
        }
    }

    pub fn with_vel(pos: f32x2, vel: f32x2) -> Particle {
        Particle{
            pos,
            vel,
            contacts: Vec::new()
        }
    }

    #[inline(always)]
    pub fn check_particle_collisions(&mut self, cell_it: &SpatialHashIter, properties: &Properties) {
        // clear last frame contacts
        self.contacts.clear();

        // make a new region from the current iterator
        // which we get to check each particle in each of those cells for collisions
        let mut col_cell_it = SpatialHashIter::new_region(&cell_it, (properties.radius * 2.0) as usize); // need to account for up to 2 * radius
        while col_cell_it.next() {
            let col_cell = col_cell_it.cell();
            for col_particle_it in col_cell {
                let col_particle = *col_particle_it;
                //println!("col cell");

                unsafe {
                    // collision check
                    let pos_delta = (*col_particle).pos - self.pos;
                    let dist_squared = length_squared(pos_delta); //(pos_delta[0] * pos_delta[0]) + (pos_delta[1] * pos_delta[1]);
                    if dist_squared <= 0.0 || dist_squared >= properties.dist_squared_max {
                        // no collision or collision with self
                        //println!(" -> NO collision");
                        continue;
                    }

                    // compute and apply velocity to each circle
                    let dist = dist_squared.sqrt();

                    let normal = pos_delta / vec2_from_single(dist);

                    // create a new contact
                    self.contacts.push(Contact {
                        normal: normal,
                        depth: dist
                    });

                    let dist_to_move = dist * 0.5;

                    // as the points get closer, the velocity increases
                    // exponentially
                    // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                    let vel_mag = 1.0 / dist_to_move;

                    let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);

                    // lose or gain energy in the outgoing velocity
                    let vel = (pos_delta * vel_m) * vec2_from_single(properties.elasticity);

                    self.vel -= vel;
                }
            }
        }
    }

    #[inline(always)]
    pub fn move_reflect(&mut self, spatial_hash: &SpatialHash, dt2: f32x2, properties: &Properties) {

        // add gravity and some damping
        self.vel += properties.gravity * dt2;
        //self.vel *= vec2_from_single(properties.damping); // * dt2;

        // TODO: iterate over contacts and modify velocity appropraitely

        // now lets look at moving the particle's position
        // get the new position of the circle
        let mut pt = self.pos + (self.vel * dt2);

        let mut collision = false;

        // for now, when we leave the map, reflect it
        if pt[1] < 0.0 {
            self.vel[1] = -self.vel[1];
            collision = true;

            //pt[1] = self.pos[1] + (self.vel[1] * dt2[0]);
        }
        else if (pt[1] as usize) >= spatial_hash.y_size {
            self.vel[1] = -self.vel[1];
            collision = true;
            //pt[1] = self.pos[1] + (self.vel[1] * dt2[0]);
        }

        if pt[0] < 0.0 {
            self.vel[0] = -self.vel[0];
            collision = true;
            //pt[0] = self.pos[0] + (self.vel[0] * dt2[0]);
        }
        else if (pt[0] as usize) >= spatial_hash.x_size {
            self.vel[0] = -self.vel[0];
            collision = true;
            //pt[0] = self.pos[0] + (self.vel[0] * dt2[0]);
        }

        if collision {
            self.vel *= vec2_from_single(properties.collision_damping);
            pt = self.pos + (self.vel * dt2);
        }

        self.pos = pt;
    }
}