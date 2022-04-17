use core_simd::*;
use crate::Shape;
use crate::Particle;

pub struct Rect {
    pub pos: f32x2,
    pub size: f32x2
}

// https://stackoverflow.com/questions/401847/circle-rectangle-collision-detection-intersection
// this also supports rotation byb rotating the particle around the rect which is in the demo code
// how to calculate how to push the particle out of the shape?
impl Shape for Rect {
    fn collide_with(&self, circle: &mut Particle) {
        const RADIUS: f32 = 1.0;

        let width = self.size[0];
        let height = self.size[1];

        let distX = (circle.pos[0] - self.pos[0] - width / 2.0).abs();
        let distY = (circle.pos[1] - self.pos[1] - height / 2.0).abs();

        if (distX > (width / 2.0 + RADIUS)) {
            return; // false;
        }
        if (distY > (height / 2.0 + RADIUS)) {
            return;// false;
        }

        if (distX <= (width / 2.0)) {
            println!("collision");
            return; // true;
        }
        if (distY <= (height / 2.0)) {
            println!("collision");
            return;// true;
        }

        let delta_x = distX - width / 2.0;
        let delta_y = distY - height / 2.0;
        let collision = delta_x * delta_x + delta_y * delta_y <= RADIUS * RADIUS;

        if (collision) {
            println!("collision");
        }
    }
}