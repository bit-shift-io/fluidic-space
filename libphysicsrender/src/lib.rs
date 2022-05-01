#![feature(portable_simd)]

pub use crate::sdl_fluid_sim_renderer::SdlFluidSimRenderer;
pub use crate::sdl_system::SdlSystem;

mod sdl_fluid_sim_renderer;
mod sdl_system;