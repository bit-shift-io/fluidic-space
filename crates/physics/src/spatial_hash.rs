use core_simd::*;
use std::cmp;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;

struct CollisionResult {
    collided: bool,
    dist_squared: f32
}

struct Buffer {
    pos: Vec<f32>,
    vel: Vec<f32>,
    bucket_sz: Vec<usize>,
}

// should we do a triple buffer to allow rendering to occur while updating the particle sim?
struct DoubleBuffer {
    current: RefCell<Buffer>,
    next: RefCell<Buffer>,
}

impl DoubleBuffer {
    pub fn new(total_size: usize, xy_size: usize) -> DoubleBuffer {
        let mut a = Buffer {
            pos: vec![0.0; total_size],
            vel: vec![0.0; total_size],
            bucket_sz: vec![0; xy_size],
        };

        let mut b = Buffer {
            pos: vec![0.0; total_size],
            vel: vec![0.0; total_size],
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

pub struct SpatialHash {
    buffer: DoubleBuffer,
    pub x_size: usize,
    pub y_size: usize,
    bucket_size: usize,
    elasticity: f32, // when intersecting what to multiply velocity by. Lower means particles can squish together more
    damping: f32, // energy loss. Higher means velocity becomes more like viscous - honey. Lower more like water
}

impl SpatialHash {
    pub fn new(x_size: usize, y_size: usize, bucket_size: usize) -> SpatialHash {
        let total_size = x_size * y_size * bucket_size;
        let xy_size = x_size * y_size;
        SpatialHash {
            buffer: DoubleBuffer::new(total_size, xy_size),
            x_size,
            y_size,
            bucket_size,
            elasticity: 1.0,
            damping: 1.0
        }
    }

    pub fn generate_random_points(&self, count: usize) -> Vec<f32> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut pts: Vec<f32> = Vec::new();

        for b in 0..count {

            let pt_x = range.sample(&mut rng) * (self.x_size as f32);
            let pt_y = range.sample(&mut rng) * (self.y_size as f32);

            pts.push(pt_x);
            pts.push(pt_y);

            println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
        }

        return pts;
    }

    // because we can only fit 'bucket_size' points per cell
    // we want a safe way to generate a set of points that will safely start the simulation off
    pub fn generate_uniform_points(&self, count_per_bucket: usize) -> Vec<f32> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut pts: Vec<f32> = Vec::new();

        for y in 0..self.y_size {
            for x in 0..self.x_size {
                for b in 0..cmp::max(1, count_per_bucket / 2) {

                    let pt_x = range.sample(&mut rng) + (x as f32);
                    let pt_y = range.sample(&mut rng) + (y as f32);

                    pts.push(pt_x);
                    pts.push(pt_y);

                    println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
                }
            }
        }

        return pts;
    }

    // simd accelerated
    pub fn add_points_simd(&mut self, pts: &Vec<f32>) {
        let mut buff = self.buffer.current.borrow_mut();

        // assert!(pts.len() multiple of 4)

        let size = pts.len() as isize;
        let chunks = size / 4;
    
        // treat f32 vector as an simd f32x4 vector
        let ptr = pts.as_ptr() as *const f32x4;

        //let simd_p_x = p_x as *mut f32x4;
    
        let size_mult: u32x4 = Simd::from_array([1, self.y_size as u32, 1, self.y_size as u32]);

        // sum excess elements that don't fit in the simd vector
        for i in 0..chunks {
            // dereferencing a raw pointer requires an unsafe block
            unsafe {
                // offset by i elements
                let fv: f32x4 = *ptr.offset(i);
                let iv: u32x4 = fv.cast::<u32>(); //i32x4::from(fv); // fv;
                let iv_size = iv * size_mult;

                // can we simd this + operation?
                // is there a conversion penalty from u32 to usize?
                // if so, can we somehow avoid it by making the vector u32 referencable? 
                let cell1 = (iv_size[0] + iv_size[1]) as usize;
                let cell2 = (iv_size[2] + iv_size[3]) as usize;

                let bucket1_length = buff.bucket_sz[cell1];
                let bucket2_length = buff.bucket_sz[cell2];

                let bucket1_cell = (cell1 * self.bucket_size) + bucket1_length;
                buff.pos[bucket1_cell] = fv[0];
                buff.pos[bucket1_cell+1] = fv[1];

                let bucket2_cell = (cell2 * self.bucket_size) + bucket2_length;
                buff.pos[bucket2_cell] = fv[2];
                buff.pos[bucket2_cell+1] = fv[3];

                println!("add point: {:?},{:?} into pts[{:?}]", fv[0], fv[1], bucket1_cell);
                println!("add point: {:?},{:?} into pts[{:?}]", fv[2], fv[3], bucket2_cell);

                assert!(buff.bucket_sz[cell1] < self.bucket_size);
                buff.bucket_sz[cell1] += 2;

                assert!(buff.bucket_sz[cell2] < self.bucket_size);
                buff.bucket_sz[cell2] += 2;
            }
        }
    }

    // bog standard
    pub fn add_points(&mut self, pts: &Vec<f32>) {
        let mut buff = self.buffer.current.borrow_mut();

        for i in (0..pts.len()).step_by(2) {
            let x = pts[i];
            let y = pts[i+1];

            //assert!(x >= 0.0);
            //assert!(y >= 0.0);

            let ix: usize = x as usize;
            let iy: usize = y as usize;

            //assert!(ix < self.x_size);
            //assert!(iy < self.y_size);

            let cell = ix + (iy * self.y_size);
            let bucket_length = buff.bucket_sz[cell];

            let bucket_cell = (cell * self.bucket_size) + bucket_length;
            buff.pos[bucket_cell] = x;
            buff.pos[bucket_cell+1] = y;

            println!("add point: {:?},{:?} into pts[{:?}]", x, y, bucket_cell);

            assert!(buff.bucket_sz[cell] < self.bucket_size);
            buff.bucket_sz[cell] += 2; // 2 floats
        }
    }

    // bog standard simulation step
    pub fn update_velocity_from_collisions(&mut self) {
        const radius: f32 = 1.0;
        const dist_squared_max: f32 = (radius + radius) * (radius + radius);
        let mut buff = self.buffer.current.borrow_mut();

        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                cell *= self.bucket_size;
                for b in (0..bucket_length).step_by(2) {
                    let bucket_cell = cell + b;

                    // get the position of the circle
                    let pt_x = buff.pos[bucket_cell];
                    let pt_y = buff.pos[bucket_cell+1];

                    // now do a radius check for other circles
                    println!("pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell);

                    //
                    // iterate over other cells & buckets to get other potential circle collisions
                    // we only check from:
                    //      1) the current cell -> left
                    //      2) the current cell -> down
                    // (i.e. the bottom right quadrant of the circle)
                    // to avoid doubling up on collision checks
                    //
                    let iy2_start = iy;
                    let iy2_end = cmp::min(self.y_size, iy + 2);

                    let ix2_start = ix;
                    let ix2_end = cmp::min(self.x_size, ix + 2);

                    for iy2 in iy2_start..iy2_end {
                        for ix2 in ix2_start..ix2_end {
                            let mut cell2 = ix2 + (iy2 * self.y_size);

                            let bucket2_length = buff.bucket_sz[cell2];
                            cell2 *= self.bucket_size;
                            for b2 in (0..bucket2_length).step_by(2) {
                                let bucket_cell2 = cell2 + b2;

                                // don't compare against ourself
                                if bucket_cell == bucket_cell2 {
                                    continue;
                                }

                                // get the position of the circle
                                let pt_x2 = buff.pos[bucket_cell2];
                                let pt_y2 = buff.pos[bucket_cell2+1];


                                // now do a radius check for other circles
                                print!("  compare against -> pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell2);

                                // compute dist between
                                let a = pt_x2 - pt_x;
                                let b = pt_y2 - pt_y;
                                let dist_squared = (a * a) + (b * b);
                                if dist_squared >= dist_squared_max {
                                    // no collision
                                    println!(" -> NO collision");
                                    continue;
                                }

                                // collision!
                                println!(" -> collision");

                                // compute and apply velocity to each circle
                                let dist = dist_squared.sqrt();
                                let dist_to_move = dist * 0.5;

                                let vel_x = (a / dist) * self.elasticity;
                                let vel_y = (b / dist) * self.elasticity;

                                buff.vel[bucket_cell] -= vel_x;
                                buff.vel[bucket_cell+1] -= vel_y;

                                buff.vel[bucket_cell2] += vel_x;
                                buff.vel[bucket_cell2+1] += vel_y;

                                // TODO: oops! we have not accounted for the existing velocity of each 
                                // circle, so they 'reflect' off of each other....

                                println!("done");
                            }
                        }
                    }

                }
            }
        }
    }

    // simd accelerated
    pub fn update_velocity_from_collisions_simd(&mut self) {
        
    }

    pub fn apply_velocity(&mut self, dt: f32) {
        let mut buff = self.buffer.current.borrow_mut();
        let mut next = self.buffer.next.borrow_mut();

        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                cell *= self.bucket_size;
                for b in (0..bucket_length).step_by(2) {
                    let bucket_cell = cell + b;

                    // get the new position of the circle
                    let mut pt_x = buff.pos[bucket_cell] + (buff.vel[bucket_cell] * dt);
                    let mut pt_y = buff.pos[bucket_cell+1] + (buff.vel[bucket_cell+1] * dt);

                    // for now, when we leave the map, reflect it
                    if pt_y < 0.0 {
                        buff.vel[bucket_cell+1] = -buff.vel[bucket_cell+1];
                        pt_y = buff.pos[bucket_cell+1] + (buff.vel[bucket_cell+1] * dt);
                    }
                    else if (pt_y as usize) >= self.y_size {
                        buff.vel[bucket_cell+1] = -buff.vel[bucket_cell+1];
                        pt_y = buff.pos[bucket_cell+1] + (buff.vel[bucket_cell+1] * dt);
                    }

                    if pt_x < 0.0 {
                        buff.vel[bucket_cell] = -buff.vel[bucket_cell];
                        pt_x = buff.pos[bucket_cell] + (buff.vel[bucket_cell] * dt);
                    }
                    else if (pt_x as usize) >= self.x_size {
                        buff.vel[bucket_cell] = -buff.vel[bucket_cell];
                        pt_x = buff.pos[bucket_cell] + (buff.vel[bucket_cell] * dt);
                    }

                    // spatial has the point
                    // and push it in to the next buffer 
                    let rx: usize = pt_x as usize;
                    let ry: usize = pt_y as usize;

                    /*
                    // for now, when we leave the map, just let the particle die
                    if rx >= self.x_size || ry >= self.y_size {
                        println!("die!");
                        continue;
                    }*/


                    //assert!(ix < self.x_size);
                    //assert!(iy < self.y_size);

                    let to_cell = rx + (ry * self.y_size);
                    let to_bucket_length = next.bucket_sz[to_cell];

                    let to_bucket_cell = (to_cell * self.bucket_size) + to_bucket_length;
                    next.pos[to_bucket_cell] = pt_x;
                    next.pos[to_bucket_cell+1] = pt_y;

                    println!("add point: {:?},{:?} into pts[{:?}]", pt_x, pt_y, to_bucket_cell);

                    assert!(next.bucket_sz[to_cell] < self.bucket_size);
                    next.bucket_sz[to_cell] += 2; // 2 floats

                    // copy velocity over also
                    next.vel[to_bucket_cell] = buff.vel[bucket_cell];
                    next.vel[to_bucket_cell+1] = buff.vel[bucket_cell+1];
                }
            }
        }
    }

    pub fn apply_velocity_simd(&mut self, dt: f32) {
    }

    // can we do a simd version of this?
    pub fn for_each_pos<F: Fn(f32, f32)>(&self, f: F) {
        let mut buff = self.buffer.current.borrow_mut();
        let mut next = self.buffer.next.borrow_mut();

        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                cell *= self.bucket_size;
                for b in (0..bucket_length).step_by(2) {
                    let bucket_cell = cell + b;

                    // get the new position of the circle
                    let pt_x = buff.pos[bucket_cell];
                    let pt_y = buff.pos[bucket_cell+1];

                    f(pt_x, pt_y);
                }
            }
        }
    }

    // clear the current buffer
    pub fn clear_next(&mut self) {
        let mut next = self.buffer.next.borrow_mut();

        // https://stackoverflow.com/questions/56114139/what-is-an-efficient-way-to-reset-all-values-of-a-vect-without-resizing-it
        // really only need to clear bucket_sz array
        // but clearing the others helps debugging
        for item in &mut next.pos { *item = 0.0; }
        for item in &mut next.vel { *item = 0.0; }
        for item in &mut next.bucket_sz { *item = 0; }

        //println!("cleared");
    }

    pub fn clear_next_simd(&mut self) {
        // TODO
    }

    pub fn swap(&mut self) {
        self.buffer.swap()
    }
}