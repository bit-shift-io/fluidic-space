use core_simd::*;
use crate::Shape;
use crate::Particle;
use crate::Vector2;

pub struct Rect {
    pub pos: f32x2,
    pub size: f32x2
}

// https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
// https://stackoverflow.com/a/1879223/500564
// https://stackoverflow.com/questions/45370692/circle-rectangle-collision-response
// 
// this also supports rotation byb rotating the particle around the rect which is in the demo code
// how to calculate how to push the particle out of the shape?
impl Shape for Rect {
    fn collide_with(&self, circle: &mut Particle) {
        const RADIUS: f32 = 1.0;
        let radius_sqrd = RADIUS * RADIUS;


        let half_size = self.size * Simd::from_array([0.5, 0.5]);

        // clamp(value, min, max) - limits value to the range min..max


        // Find the closest point to the circle within the rectangle
        let closest = circle.pos.clamp(self.pos - half_size, self.pos + half_size);
        /*
        float closestX = clamp(circle.X, rectangle.Left, rectangle.Right);
        float closestY = clamp(circle.Y, rectangle.Top, rectangle.Bottom);
        */

        // Calculate the distance between the circle's center and this closest point
        let dist_vec = Vector2::from_f32x2(circle.pos - closest);
        /*
        float distanceX = circle.X - closestX;
        float distanceY = circle.Y - closestY;
        */

        // why is horizontal_sum not working?
        let dist_m_dist = dist_vec * dist_vec;
        let dist_sqrd: f32 = dist_m_dist[0] + dist_m_dist[1]; //(dist * dist).horizontal_sum();// //(dist[0] * dist[0]) + (dist[1] * dist[1]);
        // If the distance is less than the circle's radius, an intersection occurs
        //float distanceSquared = (distanceX * distanceX) + (distanceY * distanceY);
        let collision = dist_sqrd < radius_sqrd;
        if (collision) {
            let normal = Vector2::new(-dist_vec[1], dist_vec[0]); //Simd::from_array([-dist_vec[1], dist_vec[0]]); //createVector(- dist.y, dist.x);
            /*
            let normal_angle = atan2(normal[1], normal[0]);
            let incoming_angle = atan2(circle.vel.y, circle.vel.x);
            let theta = normal_angle - incoming_angle;
            circle.vel = circle.vel.rotate(2*theta);
            */

            let dist = dist_sqrd.sqrt();
            let dist_to_push = RADIUS - dist_sqrd.sqrt();
            let push_vec = dist_vec * Simd::from_array([dist, dist]) * Simd::from_array([dist_to_push, dist_to_push]);
            circle.pos += push_vec

            //println!("collision");
            // push out to closest point
            //circle.pos = closest;
        }



/*
        // Find the closest point on the rect to the circle
        let mut point_on_rect = (circle.pos - self.pos - half_size).abs();

        //let distX = (circle.pos[0] - self.pos[0] - half_width).abs();
        //let distY = (circle.pos[1] - self.pos[1] - half_height).abs();

        if (point_on_rect[0] > (half_size[0] + RADIUS)) {
            return; // false;
        }
        if (point_on_rect[1] > (half_size[1] + RADIUS)) {
            return;// false;
        }

        if (point_on_rect[0] <= half_size[0]) {
            println!("collision");
            return; // true;
        }
        if (point_on_rect[1] <= half_size[1]) {
            println!("collision");
            return;// true;
        }

        let delta = point_on_rect - half_size;
        let dist_sqrd = (delta[0] * delta[0]) + (delta[1] * delta[1]);
        let collision = dist_sqrd <= (RADIUS * RADIUS);
/*
        let delta_x = distX - width / 2.0;
        let delta_y = distY - height / 2.0;
        let collision = delta_x * delta_x + delta_y * delta_y <= RADIUS * RADIUS;
*/
        if (collision) {
            println!("collision");
        }*/
    }
}