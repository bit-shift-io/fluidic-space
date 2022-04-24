pub use core_simd::*;
//use std::cmp;
//use rand::distributions::{Distribution, Uniform};

pub use crate::fluid_sim::fluid_sim::FluidSim;
pub use crate::fluid_sim::spatial_hash::SpatialHash;
pub use crate::fluid_sim::spatial_hash_iter::SpatialHashIter;
pub use crate::fluid_sim::particle::Particle;
pub use crate::fluid_sim::test::*;

pub use crate::fluid_sim::shape::Shape;
pub use crate::fluid_sim::rect::Rect;
pub use crate::fluid_sim::vector_2::*;

mod spatial_hash;
mod spatial_hash_iter;
mod particle;
mod test;
mod bench;
mod fluid_sim;

mod shape;
mod rect;
mod vector_2;