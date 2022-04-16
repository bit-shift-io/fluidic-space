use core_simd::*;


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
}