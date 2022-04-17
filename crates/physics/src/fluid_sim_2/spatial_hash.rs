use std::ptr;
use core_simd::*;
use crate::fluid_sim_2::particle::Particle;

/*
pub struct Hash {
    pub xy: u32x2,
    pub cell: usize,
    pub particle: *const Particle
}
*/

pub type Cell = Vec<*mut Particle>;

pub struct SpatialHash {
    pub x_size: usize,
    pub y_size: usize,
    pub cells: Vec<Cell>, // TODO: this should be a vector of vector of particles!
    //pub particleHash: Vec<Hash>

    pub size_mult: u32x2
}

impl SpatialHash {
    pub fn new(x_size: usize, y_size: usize) -> SpatialHash {
        let total_size = x_size * y_size;
        let empty_cell = Cell::new(); //vec![ptr::null(); 0]; //Vec<*const Particle>::new();

        let size_mult: u32x2 = Simd::from_array([1, y_size as u32]);
        SpatialHash{
            x_size,
            y_size,
            cells: vec![empty_cell; total_size],
            size_mult
        }
    }

    pub fn clear(&mut self) {
        /*
        unsafe {
            let vec_ptr = self.vec.as_mut_ptr();
            ptr::write_bytes(vec_ptr, 0, self.vec.len());
        }*/
        for cell in &mut self.cells {
            cell.clear();
        }
        println!("cleared");
    }

    pub fn add_particle(&mut self, particle: &mut Particle) {
        let upos: u32x2 = (*particle).pos.cast::<u32>();
        let upos_size = upos * self.size_mult;
        let cell = (upos_size[0] + upos_size[1]) as usize;
        self.cells[cell].push(particle);
    }

    pub fn add_particles(&mut self, particles: &mut Vec<Particle>) {
        //let size_mult: u32x4 = Simd::from_array([1, self.y_size as u32, 1, self.y_size as u32]);
        let particles_ptr = particles.as_mut_ptr() as *mut Particle;
        for i in 0..particles.len() {
            unsafe {
                let particle = particles_ptr.offset(i as isize);
                self.add_particle(&mut *particle);
/*
                let upos: u32x2 = (*particle).pos.cast::<u32>();
                let upos_size = upos * size_mult_x2;
                let cell = (upos_size[0] + upos_size[1]) as usize;

                self.cells[cell].push(particle);*/
            }
        }
/*
        for particle in particles {
            let upos: u32x2 = particle.pos.cast::<u32>();
            let upos_size = upos * size_mult_x2;
            let cell = (upos_size[0] + upos_size[1]) as usize;

            // https://stackoverflow.com/questions/53458784/why-is-casting-a-const-reference-directly-to-a-mutable-reference-invalid-in-rust
            let p_ptr = &particle as *const _ as *const _ as *mut Particle; //particle as *mut Particle;
            self.cells[cell].push(p_ptr);
/ *
            unsafe {
                let p: *mut Particle = &mut *particle;
                self.cells[cell].push(p);
            }* /
            / *
            particle.hash = Hash{
                xy: upos_size,
                cell: cell
            }* /
        }*/
        println!("added");
    }
}