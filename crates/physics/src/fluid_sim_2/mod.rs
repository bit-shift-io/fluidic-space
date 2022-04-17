pub use core_simd::*;
use std::cmp;
use rand::distributions::{Distribution, Uniform};

pub use crate::fluid_sim_2::fluid_sim_2::FluidSim2;
pub use crate::fluid_sim_2::spatial_hash::SpatialHash;
pub use crate::fluid_sim_2::spatial_hash_iter::SpatialHashIter;
pub use crate::fluid_sim_2::particle::Particle;
pub use crate::fluid_sim_2::test::*;

mod spatial_hash;
mod spatial_hash_iter;
mod particle;
mod test;
mod bench;
mod fluid_sim_2;
