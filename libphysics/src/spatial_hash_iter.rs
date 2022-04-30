
//use core_simd::*;
use std::num::Wrapping;
use std::cmp;
use crate::spatial_hash::*;

pub struct SpatialHashIter<'a> {
    pub spatial_hash: &'a SpatialHash,

    pub x_start: usize,
    pub x_end: usize,
    
    pub y_start: usize,
    pub y_end: usize,
    pub stride: usize,

    pub x: Wrapping<usize>,
    pub y: Wrapping<usize>,
    //pub xy: u32x2,
    pub cell: Wrapping<usize>,
}

impl<'a> SpatialHashIter<'a> {
    pub fn new(spatial_hash: &'a SpatialHash) -> SpatialHashIter<'a> {
        SpatialHashIter{
            spatial_hash,
            x_start: 0,
            x_end: spatial_hash.x_size,
            y_start: 0,
            y_end: spatial_hash.y_size,
            stride: 0,
            x: Wrapping(usize::MAX),
            y: Wrapping(0),
            cell: Wrapping(usize::MAX)
        }
    }

    pub fn new_region(spatial_hash_iter: &'a SpatialHashIter, radius: usize) -> SpatialHashIter<'a> {
        let spatial_hash = spatial_hash_iter.spatial_hash;
        let x_start = if spatial_hash_iter.x.0 < radius { 0 } else { spatial_hash_iter.x.0 - radius };
        let x_end = cmp::min(spatial_hash.x_size, spatial_hash_iter.x.0 + radius + 1);

        let y_start = if spatial_hash_iter.y.0 < radius { 0 } else { spatial_hash_iter.y.0 - radius };
        let y_end = cmp::min(spatial_hash.y_size, spatial_hash_iter.y.0 + radius + 1);

        let mut x = Wrapping(x_start);
        x -= 1;
        let y = Wrapping(y_start);

        let mut cell = Wrapping(x_start + (y.0 * spatial_hash.y_size));
        cell -= 1 as usize;

        let delta = x_end - x_start;
        let stride = spatial_hash.x_size - delta;

        SpatialHashIter{
            spatial_hash,
            x_start,
            x_end,
            y_start,
            y_end,
            stride,
            x,
            y,
            cell
        }
    }

    pub fn next(&mut self) -> bool {
        self.x += Wrapping(1);
        self.cell += Wrapping(1);
        //assert!(self.cell == Wrapping(self.x.0 + (self.y.0 * self.spatial_hash.y_size)), "Ops! Stride must be wrong");

        if self.x >= Wrapping(self.x_end) {
            self.y += Wrapping(1);
            self.cell += Wrapping(self.stride);

            if self.y >= Wrapping(self.y_end) {
                return false;
            }

            self.x = Wrapping(self.x_start);

            //assert!(self.cell == Wrapping(self.x.0 + (self.y.0 * self.spatial_hash.y_size)), "Ops! Stride must be wrong");
        }
        return true;
    }

    pub fn cell(&self) -> &Cell {
        return &self.spatial_hash.cells[self.cell.0];
    }
}