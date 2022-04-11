
use crate::FluidSim;
//use std::cell::RefCell;
use core::cell::RefMut;
use crate::fluid_sim::double_buffer::Buffer;
use core_simd::*;

pub struct Iter<'a> {
    pub fluid_sim: &'a FluidSim,
    pub ix: isize,
    pub iy: isize,
    pub i: isize,
    pub icell: isize,
    pub bucket_length: isize,
    pub ib: isize,
    pub buff: RefMut<'a, Buffer>,
    pub ibuff: usize,
}

impl<'a> Iter<'a> {
    fn new(fluid_sim: &'a FluidSim) -> Iter<'a> {
        Iter {
            fluid_sim,
            ix: -1,
            iy: 0,
            i: -1,
            icell: -(fluid_sim.bucket_size as isize),
            bucket_length: 0,
            ib: -1, // index into bucket array
            ibuff: 0, // index into buff array
            buff: fluid_sim.buffer.current.borrow_mut()
        }
    }

    /*
    // or use the copy trait?
    fn clone(&self) -> Iter<'a> {
        Iter {
            fluid_sim: self.fluid_sim,
            ix: self.ix,
            iy: self.iy,
            i: self.i,
            icell: self.icell,
            bucket_length: self.bucket_length,
            buff: self.buff //self.fluid_sim.buffer.current.borrow_mut() //: self.buff //TODO: exceptionn is here! how do we fix this?
        }
    }*/

    pub fn next(&mut self) -> bool /*Option<Self::Item>*/ {
        self.ib += 1;
        if self.ib >= self.bucket_length {
            self.ib = 0;

            self.ix += 1;
            self.i += 1;
            self.icell += self.fluid_sim.bucket_size as isize;
            if self.ix >= self.fluid_sim.x_size as isize {
                self.iy += 1;
                if self.iy >= self.fluid_sim.y_size as isize {
                    return false;
                }

                self.ix = 0;
            }
        }

        // ignore empty buckets/cells
        self.bucket_length = self.buff.bucket_sz[self.i as usize] as isize;
        if self.bucket_length <= 0 {
            return self.next();
        }

        self.ibuff = (self.icell + self.ib) as usize;

        //let dup = self.clone();
        return true; //Some(true)
    }

    /*
    pub fn vel(&mut self) {
        return self.buff.vel.offset(self.buff.ibuff);
    }

    pub fn pos(&mut self) {
        return self.buff.pos.offset(self.buff.ibuff);
    }*/
}


/*
impl<'a> Iterator for Iter<'a> {
    type Item = bool; //Iter<'a>; //Cell<'a>; //&'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.ix += 1;
        self.i += 1;
        self.icell += self.fluid_sim.bucket_size as isize;
        if self.ix >= self.fluid_sim.x_size as isize {
            self.iy += 1;
            if self.iy >= self.fluid_sim.y_size as isize {
                return None
            }

            self.ix = 0;
        }

        // ignore empty buckets/cells
        self.bucket_length = self.buff.bucket_sz[self.icell as usize] as isize;
        if self.bucket_length <= 0 {
            return self.next();
        }

        //let dup = self.clone();
        Some(true)
    }
}*/

impl FluidSim {
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(&self)
        /*
        Iter {
            fluid_sim: &self,
            ix: -1,
            iy: 0,
            i: -1,
            icell: -(self.bucket_size as isize),
            bucket_size: 0,
            buff: self.buffer.current.borrow_mut()
        }*/
    }
}