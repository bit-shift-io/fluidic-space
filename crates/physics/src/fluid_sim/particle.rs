use core_simd::*;
use crate::fluid_sim::spatial_hash::SpatialHash;
use crate::fluid_sim::fluid_sim::Properties;
use crate::*;

#[derive(Clone)]
pub struct Contact {
    //pub pos: f32x2,
    pub normal: f32x2,
    pub depth: f32,
    pub particle: *mut Particle
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
    pub fn check_particle_collisions(&mut self, cell_it: &SpatialHashIter, properties: &Properties, dt: f32x2) {
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
                        depth: dist,
                        particle: col_particle
                    });

                    // TODO: should this conntriute towards an 'instantanous push' amount
                    // which is different to velocity?
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
    pub fn update_velocity(&mut self, spatial_hash: &SpatialHash, properties: &Properties, dt: f32x2) {
        // add gravity
        self.vel += properties.gravity * dt;

        // iterate over contacts and modify velocity
        for contact in &self.contacts {
            // create tangent from normal, then project velocity into the tangent
            // to stop/negate any velocity towards the normal
            let tangent = vec2(contact.normal[1], contact.normal[0]);
            let projected_vel = project(self.vel, tangent);
            let transfer_vel = self.vel - projected_vel;
            self.vel = projected_vel;

            // we compute the velocity loss here and apply that velocity to the other particle
            // scenario: particle 1 is not moving, is hit by particle 2 which is moving at high speed
            unsafe {
                (*contact.particle).vel += transfer_vel * vec2_from_single(0.9); // energy loss on collision
            }
        }
    }

    #[inline(always)]
    pub fn move_pos(&mut self, spatial_hash: &SpatialHash, properties: &Properties, dt: f32x2) {
        // now lets look at moving the particle's position
        // get the new position of the circle
        let mut pt = self.pos + (self.vel * dt);

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
            pt = self.pos + (self.vel * dt);
        }

        self.pos = pt;
    }
}