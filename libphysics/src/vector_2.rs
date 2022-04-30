pub use core_simd::*;

/*
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
*/

pub fn vec2(x: f32, y: f32) -> f32x2 {
    return Simd::from_array([x, y])
}

pub fn vec2_from_single(v: f32) -> f32x2 {
    return Simd::from_array([v, v])
}

pub fn length_squared(v: f32x2) -> f32 {
    let dist_m_dist = v * v;
    let dist_sqrd: f32 = dist_m_dist[0] + dist_m_dist[1]; //(dist * dist).horizontal_sum();// //(dist[0] * dist[0]) + (dist[1] * dist[1]);
    return dist_sqrd;
}

pub fn dot(a: f32x2, b: f32x2) -> f32 {
    let m = a * b;
    return m[0] + m[1];
}

// https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
pub fn reflect(v: f32x2, n: f32x2) -> f32x2 {
    // v = incoming velocity
    // n = normal
    // r = reflected vector
    // r = v - 2 * (v dot n) * n
    let dot = dot(v, n);
    let dot2 = vec2_from_single(2.0 * dot) * n; 
    let r = v - dot2;
    return r;
}

// project a onto b
pub fn project(a: f32x2, b: f32x2) -> f32x2 {
    // https://www.omnicalculator.com/math/vector-projection
    // p = (a·b / b·b) * b
    let p = vec2_from_single(dot(a, b) / dot(b, b)) * b;
    return p;
}

// counterclockwise rotation
pub fn rotate_vector(v: f32x2, angle_rad: f32) -> f32x2 {
    let c = angle_rad.cos();
    let s = angle_rad.sin();
    let rotated = vec2(
        (v[0] * c) - (v[1] * s),
        (v[0] * s) + (v[1] * c)
    );
    return rotated;
}

// rotate pt around origin by a given angle (in radians)
pub fn rotate_point_around(pt: f32x2, origin: f32x2, angle_rad: f32) -> f32x2 {
    let mut delta = pt - origin; 
    let rotated = rotate_vector(delta, angle_rad);
    let new_pt = origin + rotated;
    return new_pt;
}