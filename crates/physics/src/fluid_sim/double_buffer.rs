use core_simd::*;
use std::cell::RefCell;

pub struct Buffer {
    pub pos: Vec<f32x2>,
    pub vel: Vec<f32x2>,
    pub bucket_sz: Vec<usize>,
}

// should we do a triple buffer to allow rendering to occur while updating the particle sim?
pub struct DoubleBuffer {
    pub current: RefCell<Buffer>,
    pub next: RefCell<Buffer>,
}

impl DoubleBuffer {
    pub fn new(total_size: usize, xy_size: usize) -> DoubleBuffer {
        let zero = Simd::from_array([0.0, 0.0]);

        let mut a = Buffer {
            pos: vec![zero; total_size],
            vel: vec![zero; total_size],
            bucket_sz: vec![0; xy_size],
        };

        let mut b = Buffer {
            pos: vec![zero; total_size],
            vel: vec![zero; total_size],
            bucket_sz: vec![0; xy_size],
        };

        let mut db = DoubleBuffer {
            current: RefCell::new(a),
            next: RefCell::new(b)
        };
        db.swap();
        return db;
    }

    pub fn swap(&mut self) {
        self.current.swap(&self.next);
    }
}