use core_simd::*;
use std::cmp;
use rand::distributions::{Distribution, Uniform};

mod double_buffer;
use crate::fluid_sim::double_buffer::DoubleBuffer;

pub use crate::fluid_sim::test::*;

pub struct FluidSim {
    pub buffer: DoubleBuffer,
    pub x_size: usize,
    pub y_size: usize,
    pub bucket_size: usize,
    pub collision_energy_loss: f32, // when colliding, energy loss on velocity
    pub elasticity: f32, // when intersecting what to multiply velocity by. Lower means particles can squish together more
    pub damping: f32, // energy loss. Higher means velocity becomes more like viscous - honey. Lower more like water
}

impl FluidSim {
    pub fn new(x_size: usize, y_size: usize, bucket_size: usize) -> FluidSim {
        let total_size = x_size * y_size * bucket_size;
        let xy_size = x_size * y_size;
        FluidSim {
            buffer: DoubleBuffer::new(total_size, xy_size),
            x_size,
            y_size,
            bucket_size,
            collision_energy_loss: 1.0,
            elasticity: 1.0,
            damping: 1.0
        }
    }

    pub fn generate_random_points(&self, count: usize) -> Vec<f32x2> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut pts: Vec<f32x2> = Vec::new();

        for b in 0..count {

            let pt_x = range.sample(&mut rng) * (self.x_size as f32);
            let pt_y = range.sample(&mut rng) * (self.y_size as f32);

            pts.push(Simd::from_array([pt_x, pt_y]));

            //println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
        }

        return pts;
    }

    /*
    // because we can only fit 'bucket_size' points per cell
    // we want a safe way to generate a set of points that will safely start the simulation off
    pub fn generate_uniform_points(&self, count_per_bucket: usize) -> Vec<f32x2> {
        let range = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut pts: Vec<f32> = Vec::new();

        for y in 0..self.y_size {
            for x in 0..self.x_size {
                for b in 0..count_per_bucket {

                    let pt_x = range.sample(&mut rng) + (x as f32);
                    let pt_y = range.sample(&mut rng) + (y as f32);

                    pts.push(Simd::from_array([pt_x, pt_y]));


                    //println!("pt-{:?}: {:?},{:?}", pts.len() / 2, pt_x, pt_y);
                }
            }
        }

        return pts;
    }*/

    // simd accelerated
    pub fn add_points_simd(&mut self, pts: &Vec<f32x2>) {
        let mut buff = self.buffer.current.borrow_mut();

        let size = pts.len() as isize;
        let chunks = size / 2;
    
        // treat f32 vector as an simd f32x4 vector
        let ptr = pts.as_ptr() as *const f32x4;
    
        let size_mult: u32x4 = Simd::from_array([1, self.y_size as u32, 1, self.y_size as u32]);
        let size_mult_x2: u32x2 = Simd::from_array([1, self.y_size as u32]);

        // sum excess elements that don't fit in the simd vector
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            for i in 0..chunks {
            
                // offset by i elements
                let fv: f32x4 = *ptr.offset(i);
                let iv: u32x4 = fv.cast::<u32>();
                let f32ptr = ptr.offset(i) as *const f32x2;
                let iv_size = iv * size_mult;

                //simd_swizzle!(iv_size, [0, 1, 0, 0]);
                //simd_swizzle!(iv_size, [2, 3, 0, 0]);

                //let fv2: f32x2 = fv.cast::<f32x2>();

                // can we simd this + operation?
                // is there a conversion penalty from u32 to usize?
                // if so, can we somehow avoid it by making the vector u32 referencable? 
                let cell1 = (iv_size[0] + iv_size[1]) as usize;
                let cell2 = (iv_size[2] + iv_size[3]) as usize;

                let bucket1_length = buff.bucket_sz[cell1];
                let bucket2_length = buff.bucket_sz[cell2];

                if bucket1_length < self.bucket_size {
                    let bucket1_cell = (cell1 * self.bucket_size) + bucket1_length;

                    buff.pos[bucket1_cell] = *f32ptr.offset(0); //Simd::from_array([fv[0], fv[1]]);
                    //buff.pos[bucket1_cell+1] = fv[1];

                    //assert!(buff.bucket_sz[cell1] < self.bucket_size);
                    buff.bucket_sz[cell1] += 1;

                    //println!("add point: {:?},{:?} into pts[{:?}]", fv[0], fv[1], bucket1_cell);
                } else {
                    println!("can't add particle!");
                }

                if bucket2_length < self.bucket_size {
                    let bucket2_cell = (cell2 * self.bucket_size) + bucket2_length;
                    
                    buff.pos[bucket2_cell] = *f32ptr.offset(1); //Simd::from_array([fv[2], fv[3]]); //f2v[1];
                    //buff.pos[bucket2_cell+1] = fv[3];

                    //assert!(buff.bucket_sz[cell2] < self.bucket_size);
                    buff.bucket_sz[cell2] += 1;

                    //println!("add point: {:?},{:?} into pts[{:?}]", fv[2], fv[3], bucket2_cell);
                } else {
                    println!("can't add particle!");
                }  
            }          
        }

        // handle excess elements that don't fit in the simd vector
        // dereferencing a raw pointer requires an unsafe block
        unsafe {
            for i in (2 * chunks)..size {
                let f32ptr = ptr.offset(i) as *const f32x2;
                let fv: f32x2 = *f32ptr;
                let iv: u32x2 = fv.cast::<u32>();
                let iv_size = iv * size_mult_x2;

                let cell1 = (iv_size[0] + iv_size[1]) as usize;

                let bucket1_length = buff.bucket_sz[cell1];

                if bucket1_length < self.bucket_size {
                    let bucket1_cell = (cell1 * self.bucket_size) + bucket1_length;

                    buff.pos[bucket1_cell] = fv; //Simd::from_array([fv[0], fv[1]]);
                    //buff.pos[bucket1_cell+1] = fv[1];

                    //assert!(buff.bucket_sz[cell1] < self.bucket_size);
                    buff.bucket_sz[cell1] += 1;

                    //println!("add point: {:?},{:?} into pts[{:?}]", fv[0], fv[1], bucket1_cell);
                } else {
                    println!("can't add particle!");
                }
            }
        }
    }

    // bog standard
    pub fn add_points(&mut self, pts: &Vec<f32x2>) {
        let mut buff = self.buffer.current.borrow_mut();

        for i in 0..pts.len() {
            let pt = pts[i];
            let x = pt[0];
            let y = pt[1];

    
            //let (x, y) = pts[i];
            //let x = pts[i];
            //let y = pts[i+1];

            //assert!(x >= 0.0);
            //assert!(y >= 0.0);

            let ix: usize = x as usize;
            let iy: usize = y as usize;

            //assert!(ix < self.x_size);
            //assert!(iy < self.y_size);

            let cell = ix + (iy * self.y_size);
            let bucket_length = buff.bucket_sz[cell];
            if bucket_length >= self.bucket_size {
                println!("can't add particle!");
                continue;
            }

            let bucket_cell = (cell * self.bucket_size) + bucket_length;
            buff.pos[bucket_cell] = pt;
            //buff.pos[bucket_cell+1] = y;

            //println!("add point: {:?},{:?} into pts[{:?}]", x, y, bucket_cell);

            assert!(buff.bucket_sz[cell] < self.bucket_size);
            buff.bucket_sz[cell] += 1;
        }
    }

    // bog standard simulation step
    pub fn update_velocity_from_collisions(&mut self) {
        const radius: f32 = 1.0;
        const dist_squared_max: f32 = (radius + radius) * (radius + radius);
        let mut buff = self.buffer.current.borrow_mut();

        // TODO: roll these loops into 1
        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                if bucket_length <= 0 {
                    continue;
                }

                const cells_to_scan: usize = 3; // need to account for up to 2 * radius

                let iy2_start = if iy < cells_to_scan { 0 } else { iy - cells_to_scan }; //cmp::max(iy - cells_to_scan, 0);
                let iy2_end = cmp::min(self.y_size, iy + cells_to_scan);

                let ix2_start = if ix < cells_to_scan { 0 } else { ix - cells_to_scan }; //cmp::max(ix - cells_to_scan, 0);
                let ix2_end = cmp::min(self.x_size, ix + cells_to_scan);
                
                cell *= self.bucket_size;
                for b in 0..bucket_length {
                    let bucket_cell = cell + b;

                    // get the position of the circle
                    let pt = buff.pos[bucket_cell];
                    //let pt_y = buff.pos[bucket_cell+1];

                    // now do a radius check for other circles
                    //println!("pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell);

                    //
                    // iterate over other cells & buckets to get other potential circle collisions
                    // we only check from:
                    //      1) the current cell -> left
                    //      2) the current cell -> down
                    // (i.e. the bottom right quadrant of the circle)
                    // to avoid doubling up on collision checks
                    //
                    // TODO: there are still bugs here allowing for 2 circles to get near each other
                    // without collision detection until BAM! collision with high energy
                    //
                    

                    for iy2 in iy2_start..iy2_end {
                        for ix2 in ix2_start..ix2_end {
                            let mut cell2 = ix2 + (iy2 * self.y_size);

                            let bucket2_length = buff.bucket_sz[cell2];
                            cell2 *= self.bucket_size;
                            for b2 in 0..bucket2_length {
                                let bucket_cell2 = cell2 + b2;

                                // don't compare against ourself
                                if bucket_cell == bucket_cell2 {
                                    continue;
                                }

                                // get the position of the circle
                                let pt2 = buff.pos[bucket_cell2];
                                //let pt_y2 = buff.pos[bucket_cell2+1];


                                // now do a radius check for other circles
                                //print!("  compare against -> pt: {:?},{:?} from pts[{:?}]", pt_x, pt_y, bucket_cell2);

                                // compute dist between
                                let a = pt2 - pt;
                                let dist_squared = (a[0] * a[0]) + (a[1] * a[1]);
                                if dist_squared >= dist_squared_max {
                                    // no collision
                                    //println!(" -> NO collision");
                                    continue;
                                }

                                // collision!
                                //println!(" -> collision");

                                // compute and apply velocity to each circle
                                let dist = dist_squared.sqrt();
                                let dist_to_move = dist * 0.5;

                                /*
                                if (dist_to_move < 0.5) {
                                    println!("to much intersection")
                                }*/

                                // as the points get closer, the velocity increases
                                // exponentially
                                // https://www.wolframalpha.com/input?i2d=true&i=plot+Divide%5B1%2Cx%5D
                                let mut vel_mag = 1.0 / dist_to_move;

                                let vel_m: f32x2 = Simd::from_array([vel_mag, vel_mag]);

                                // lose or gain energy in the outgoing velocity
                                let vel = (a * vel_m) * Simd::from_array([self.elasticity, self.elasticity]);
                                //let vel_y = (a[1] * vel_mag) * self.elasticity;

                                // loose some energy from the incoming velocity
                                //buff.vel[bucket_cell] *= self.collision_energy_loss;
                                //buff.vel[bucket_cell+1] *= self.collision_energy_loss;

                                //buff.vel[bucket_cell2] *= self.collision_energy_loss;
                                //buff.vel[bucket_cell2+1] *= self.collision_energy_loss;

                                buff.vel[bucket_cell] -= vel;
                                //buff.vel[bucket_cell+1] -= vel_y;

                                //buff.vel[bucket_cell2] += vel_x;
                                //buff.vel[bucket_cell2+1] += vel_y;

                                //println!("done");
                            }
                        }
                    }

                }
            }
        }
    }

    // simd accelerated
    pub fn update_velocity_from_collisions_simd(&mut self) {
        // TODO:
        self.update_velocity_from_collisions();
    }

    // add uniform velocity to velocity of particles e.g. gravity
    pub fn add_uniform_velocity(&mut self, vel: f32x2) {
        let mut buff = self.buffer.current.borrow_mut();

        // TODO: roll into 1 loop
        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                if bucket_length <= 0 {
                    continue;
                }

                cell *= self.bucket_size;
                for b in 0..bucket_length {
                    let bucket_cell = cell + b;

                    buff.vel[bucket_cell] += vel;
                    //buff.vel[bucket_cell+1] += vel_y;
                }
            }
        }
    }

    // this seems slower?!
    // how to make this f32x4?
    pub fn add_uniform_velocity_simd(&mut self, vel: f32x2) {
        let mut buff = self.buffer.current.borrow_mut();

        for cell in 0..(self.x_size * self.y_size) {
            // for each bucket within the x/y cell....
            let bucket_length = buff.bucket_sz[cell];
            if bucket_length <= 0 {
                continue;
            }

            for b in cell..(cell + bucket_length) {
                buff.vel[b] += vel;
            }
        }
    }

    pub fn apply_velocity(&mut self, dt: f32) {
        let mut buff = self.buffer.current.borrow_mut();
        let mut next = self.buffer.next.borrow_mut();

        let v_dt: f32x2 = Simd::from_array([dt, dt]);
        let v_damping: f32x2 = Simd::from_array([self.damping, self.damping]);

        // TODO: roll this loop into 1
        // for each x/y cell....
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                if bucket_length <= 0 {
                    continue;
                }

                cell *= self.bucket_size;
                for b in 0..bucket_length {
                    let bucket_cell = cell + b;

                    // get the new position of the circle
                    let mut pt = buff.pos[bucket_cell] + (buff.vel[bucket_cell] * v_dt);
                    //let mut pt_y = buff.pos[bucket_cell+1] + (buff.vel[bucket_cell+1] * dt);

                    // for now, when we leave the map, reflect it
                    if pt[1] < 0.0 {
                        buff.vel[bucket_cell][1] = -buff.vel[bucket_cell][1];
                        pt[1] = buff.pos[bucket_cell][1] + (buff.vel[bucket_cell][1] * dt);
                    }
                    else if (pt[1] as usize) >= self.y_size {
                        buff.vel[bucket_cell][1] = -buff.vel[bucket_cell][1];
                        pt[1] = buff.pos[bucket_cell][1] + (buff.vel[bucket_cell][1] * dt);
                    }

                    if pt[0] < 0.0 {
                        buff.vel[bucket_cell][0] = -buff.vel[bucket_cell][0];
                        pt[0] = buff.pos[bucket_cell][0] + (buff.vel[bucket_cell][0] * dt);
                    }
                    else if (pt[0] as usize) >= self.x_size {
                        buff.vel[bucket_cell][0] = -buff.vel[bucket_cell][0];
                        pt[0] = buff.pos[bucket_cell][0] + (buff.vel[bucket_cell][0] * dt);
                    }

                    // spatial has the point
                    // and push it in to the next buffer 
                    let rx: usize = pt[0] as usize;
                    let ry: usize = pt[1] as usize;

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
                    next.pos[to_bucket_cell] = pt;
                    //next.pos[to_bucket_cell+1] = pt_y;

                    //println!("add point: {:?},{:?} into pts[{:?}]", pt_x, pt_y, to_bucket_cell);

                    assert!(next.bucket_sz[to_cell] < self.bucket_size, "Too many points in the same bucket, either reduce elasticity or increase bucket size");
                    next.bucket_sz[to_cell] += 1;

                    // copy velocity over also
                    next.vel[to_bucket_cell] = buff.vel[bucket_cell] * v_damping;
                    //next.vel[to_bucket_cell+1] = buff.vel[bucket_cell+1] * self.damping;
                }
            }
        }
    }

    pub fn apply_velocity_simd(&mut self, dt: f32) {
        // TODO:
        self.apply_velocity(dt);
    }

    // THIS IS HORRIBLY SLOW! rethink how we do this
    // can we do a simd version of this?
    pub fn for_each_pos<F: Fn(f32x2)>(&self, f: F) {
        let mut buff = self.buffer.current.borrow_mut();
        let mut next = self.buffer.next.borrow_mut();

        // TODO: unroll this loop
        // for each x/y cell.... fix this iteration down 2 1 loop
        for iy in 0..self.y_size {
            for ix in 0..self.x_size {
                let mut cell = ix + (iy * self.y_size);

                // for each bucket within the x/y cell....
                let bucket_length = buff.bucket_sz[cell];
                cell *= self.bucket_size;
                for b in 0..bucket_length {
                    let bucket_cell = cell + b;

                    // get the new position of the circle
                    let pos = buff.pos[bucket_cell];
                    //let pt_y = buff.pos[bucket_cell+1];

                    f(pos);
                }
            }
        }
    }

    /*
    // THIS IS HORRIBLY SLOW! rethink how we do this
    pub fn for_each_pos_simd<F: Fn(f32x2)>(&self, f: F) {
        // TODO: process 2 at once!
        self.for_each_pos(f);
    }*/

    // clear the current buffer
    pub fn clear_next(&mut self, full_clear: bool) {
        let mut next = self.buffer.next.borrow_mut();

        // https://stackoverflow.com/questions/56114139/what-is-an-efficient-way-to-reset-all-values-of-a-vect-without-resizing-it
        // really only need to clear bucket_sz array
        // but clearing the others helps debugging
        for item in &mut next.bucket_sz { *item = 0; }

        // should only need to specify full_clear to help when debubgging
        if full_clear {
            let clear_value: f32x2 = Simd::from_array([0.0, 0.0]);

            for item in &mut next.pos { *item = clear_value; }
            for item in &mut next.vel { *item = clear_value; }
        }

        //println!("cleared");
    }

    pub fn clear_next_simd(&mut self, full_clear: bool) {
        let mut next = self.buffer.next.borrow_mut();

        let size = next.bucket_sz.len() as isize; //xy_size as isize;
        let sizeof_usize = std::mem::size_of::<usize>() as isize;
        let sizeof_usize2 = std::mem::size_of::<u32x4>() as isize;
        let num_usize_per_simd = sizeof_usize2 / sizeof_usize;
        let chunks = size / num_usize_per_simd;
    
        let ptr_m: *mut usize = next.bucket_sz.as_mut_ptr();

        // treat vector as an simd u32x4 vector
        let ptr = next.bucket_sz.as_mut_ptr() as *mut u32x4;
        let clear_value: u32x4 = Simd::from_array([0, 0, 0, 0]);

        for i in 0..chunks {
            // dereferencing a raw pointer requires an unsafe block
            unsafe {
                // offset by i elements  
                *ptr.offset(i) = clear_value;       
            }
        }

        // excess elements that don't fit in the simd vector
        for i in (4 * chunks)..size {
            // dereferencing a raw pointer requires an unsafe block
            unsafe {
                // offset by i elements
                *ptr_m.offset(i) = 0;
            }
        }
    }

    pub fn swap(&mut self) {
        self.buffer.swap()
    }
}


mod fluid_sim_iter;
mod test;
mod bench;