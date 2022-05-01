use core_simd::*;
use crate::shape::Shape;
use crate::particle::Particle;
use crate::vector_2::*;
use crate::fluid_sim::Properties;

pub struct Rect {
    pub pos: f32x2,
    pub size: f32x2,
    pub rotation: f32 // radians
}

// https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
// https://stackoverflow.com/a/1879223/500564
// https://stackoverflow.com/questions/45370692/circle-rectangle-collision-response
// 
// this also supports rotation byb rotating the particle around the rect which is in the demo code
// how to calculate how to push the particle out of the shape?
impl Shape for Rect {
    fn collide_with(&self, circle: &mut Particle, _properties: &Properties) {
        let circle_pos = rotate_point_around(circle.pos, self.pos, -self.rotation); 

        const RADIUS: f32 = 1.0;
        let radius_sqrd = RADIUS * RADIUS;

        let half_size = self.size * Simd::from_array([0.5, 0.5]);

        // Find the closest point to the circle within the rectangle
        let closest = circle_pos.clamp(self.pos - half_size, self.pos + half_size);

        // Calculate the distance between the circle's center and this closest point
        let dist_vec = circle_pos - closest;
        let dist_sqrd: f32 = length_squared(dist_vec);

        // If the distance is less than the circle's radius, an intersection occurs
        let collision = dist_sqrd < radius_sqrd;
        if collision {
            // https://stackoverflow.com/a/45373126/500564
            // second option:
            // get the velocity along the tangent and then subtracting twice this value from the circle velocity

            let normal = dist_vec / vec2_from_single(dist_sqrd.sqrt()); // this will be our normal

            // rotate the normal back into world space
            let unrotated_normal = rotate_vector(normal, self.rotation);
            circle.vel = reflect(circle.vel, unrotated_normal);
            //circle.vel *= vec2_from_single(properties.collision_damping); // a collision occured, so sape some energy

            let dist = dist_sqrd.sqrt();
            let dist_to_push = RADIUS - dist_sqrd.sqrt();
            let push_vec = dist_vec * vec2(dist, dist) * vec2(dist_to_push, dist_to_push);

            let unrotated_push_vec = rotate_vector(push_vec, self.rotation); // rotate this vector back into the circle world space
            circle.pos += unrotated_push_vec;
            //circle.pos += push_vec
        }
    }
}