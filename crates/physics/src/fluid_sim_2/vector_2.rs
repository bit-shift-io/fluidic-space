pub use core_simd::*;

//pub type Vector2 = f32x2;

pub struct Vector2(f32x2);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 {
            0: Simd::from_array([x, y])
        }
    }

    pub fn from_f32x2(v: f32x2) -> Vector2 {
        Vector2 {
            0: v
        }
    }

    pub fn length_squared(&self) -> f32 {
        let dist_m_dist = self.0 * self.0;
        let dist_sqrd: f32 = dist_m_dist[0] + dist_m_dist[1]; //(dist * dist).horizontal_sum();// //(dist[0] * dist[0]) + (dist[1] * dist[1]);
        return dist_sqrd;
    }
}
