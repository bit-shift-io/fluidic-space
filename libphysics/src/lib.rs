#![feature(portable_simd)]
#![feature(test)]

pub use core_simd::*;

pub use crate::fluid_sim::FluidSim;
pub use crate::spatial_hash::SpatialHash;
pub use crate::spatial_hash_iter::SpatialHashIter;
pub use crate::particle::Particle;
pub use crate::test::*;

pub use crate::shape::Shape;
pub use crate::rect::Rect;
pub use crate::vector_2::*;

mod spatial_hash;
mod spatial_hash_iter;
mod particle;
mod test;
mod bench;
mod fluid_sim;
mod shape;
mod rect;
mod vector_2;