use core_simd::*;

pub struct SpatialHash {
    pts: Vec<f32>,
    bucket_sz: Vec<usize>,
    x_size: usize,
    y_size: usize,
    bucket_size: usize
}

impl SpatialHash {
    pub fn new(x_size: usize, y_size: usize, bucket_size: usize) -> SpatialHash {
        let total_size = x_size * y_size * bucket_size;
        let xy_size = x_size * y_size;
        SpatialHash {
            pts: vec![0.0; total_size],
            bucket_sz: vec![0; xy_size],
            x_size,
            y_size,
            bucket_size
        }
    }

    // simd accelerated
    pub fn add_points_simd(&mut self, pts: &Vec<f32>) {
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

                let bucket1_length = self.bucket_sz[cell1];
                let bucket2_length = self.bucket_sz[cell2];

                let bucket1_cell = cell1 + bucket1_length;
                self.pts[bucket1_cell] = fv[0];
                self.pts[bucket1_cell+1] = fv[1];

                let bucket2_cell = cell2 + bucket2_length;
                self.pts[bucket2_cell] = fv[2];
                self.pts[bucket2_cell+1] = fv[3];

                println!("add point: {:?},{:?} into pts[{:?}]", fv[0], fv[1], bucket1_cell);
                println!("add point: {:?},{:?} into pts[{:?}]", fv[2], fv[3], bucket2_cell);

                //assert!(self.bucket_sz[cell1] < self.bucket_size);
                self.bucket_sz[cell1] += 2;

                //assert!(self.bucket_sz[cell2] < self.bucket_size);
                self.bucket_sz[cell2] += 2;
            }
        }
    }

    // bog standard
    pub fn add_points(&mut self, pts: &Vec<f32>) {
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
            let bucket_length = self.bucket_sz[cell];

            let bucket_cell = cell + bucket_length;
            self.pts[bucket_cell] = x;
            self.pts[bucket_cell+1] = y;

            println!("add point: {:?},{:?} into pts[{:?}]", x, y, bucket_cell);

            //assert!(self.bucket_sz[cell] < self.bucket_size);
            self.bucket_sz[cell] += 2; // 2 floats
        }
    }

    // bog standard simulation step
    pub fn sim_step(&mut self, dt: f32) {
        // for each x/y cell....
        let xy_size = self.x_size * self.y_size;
        for i in 0..xy_size {
            let cell = i * self.bucket_size;

            // for each bucket within the x/y cell....
            let bucket_length = self.bucket_sz[i];
            for b in (0..bucket_length).step_by(2) {
                let bucket_cell = cell + b;

                // get the position of the circle
                let x = self.pts[bucket_cell];
                let y = self.pts[bucket_cell+1];

                // now do a radius check for other circles
                println!("pt: {:?},{:?}", x, y);
            }
        }
    }

    // simd accelerated
    pub fn sim_step_simd(&mut self, dt: f32) {
    }
}
