use core_simd::*;
use crate::fluid_sim_2::spatial_hash::SpatialHash;
use crate::fluid_sim_2::fluid_sim_2::Properties;
use crate::*;

// TODO: fluid sim 3 with pos and vel split into seperate arrays of f32x2 for simd improvmenets
#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: f32x2,
    pub vel: f32x2
}

impl Particle {
    pub fn new(pos: f32x2) -> Particle {
        Particle{
            pos,
            vel: Simd::from_array([0.0, 0.0])
        }
    }

    pub fn with_vel(pos: f32x2, vel: f32x2) -> Particle {
        Particle{
            pos,
            vel
        }
    }

    #[inline(always)]
    pub fn move_reflect(&mut self, spatial_hash: &SpatialHash, dt2: f32x2, properties: &Properties) {
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