
use crate::FluidSim;

// iterators

pub struct Iter<'a> {
    pub fluid_sim: &'a FluidSim,
    pub ix: isize,
    pub iy: isize,
    pub i: isize,
    pub icell: isize
}

impl<'a> Iterator for Iter<'a> {
    type Item = Iter<'a>; //Cell<'a>; //&'a Cell;

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

        let dup = Iter{
            fluid_sim: self.fluid_sim,
            ix: self.ix,
            iy: self.iy,
            i: self.i,
            icell: self.icell
        };
        Some(dup)
    }
}

impl FluidSim {
    pub fn iter(&self) -> Iter {
        Iter {
            fluid_sim: &self,
            ix: -1,
            iy: 0,
            i: -1,
            icell: -(self.bucket_size as isize)
        }
    }
}