use crate::FluidSim;
use crate::Particle;
use crate::Rect;

pub trait FluidSimRenderer {
    //fn draw_particle(&self, particle: &Particle);
    //fn draw_rect(&self, rect: &Rect);
    fn draw(&self);
}