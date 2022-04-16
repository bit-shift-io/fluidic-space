
use core_simd::*;
use std::num::Wrapping;
use crate::fluid_sim_2::spatial_hash::*;

pub struct SpatialHashIter<'a> {
    pub spatial_hash: &'a SpatialHash,
    pub x: Wrapping<usize>,
    pub y: Wrapping<usize>,
    //pub xy: u32x2,
    pub cell: Wrapping<usize>,
}

impl<'a> SpatialHashIter<'a> {
    pub fn new(spatial_hash: &'a SpatialHash) -> SpatialHashIter<'a> {
        SpatialHashIter{
            spatial_hash,
            x: Wrapping(usize::MAX),
            y: Wrapping(0),
            //xy: Simd::from_array([u32::MAX, u32::MAX]),
            cell: Wrapping(usize::MAX)
        }
    }

    pub fn next(&mut self) -> bool {
        self.x += Wrapping(1);
        self.cell += Wrapping(1);
        if self.x >= Wrapping(self.spatial_hash.x_size) {
            self.y += Wrapping(1);
            if self.y >= Wrapping(self.spatial_hash.y_size) {
                return false;
            }

            self.x = Wrapping(0);
        }
        return true;
    }

    pub fn cell(&self) -> &Cell {
        return &self.spatial_hash.cells[self.cell.0];
    }
}