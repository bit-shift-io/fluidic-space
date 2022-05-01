use sdl2::rect::Point;
use sdl2::rect::Rect as SDLRect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::WindowPos;

use libphysics::FluidSimRenderer;
use libphysics::Particle;
use libphysics::Rect;
use libphysics::FluidSim;
use libphysics::*;

pub struct SDLFluidSimRenderer/*<'a>*/ {
    //fluid_sim: &'a mut FluidSim,
    //canvas: &'a mut WindowCanvas
}

impl SDLFluidSimRenderer/*<'_>*/ {
    pub fn new/*<'a>*/(/*fluid_sim: &'a mut FluidSim, canvas: &'a mut WindowCanvas*/) -> SDLFluidSimRenderer/*<'a>*/ {
        SDLFluidSimRenderer {
            //fluid_sim,
            //canvas
        }
    }
}

fn draw_rect_rotate(canvas: &mut WindowCanvas, rect: &libphysics::Rect, scale: f32, offset: f32x2) { 
    let half_size = rect.size * vec2_from_single(0.5) * vec2_from_single(scale);

    let pos = (rect.pos * vec2_from_single(scale)) + offset;
    let top_left = pos - half_size;
    let bottom_right = pos + half_size;
    let top_right = vec2(bottom_right[0], top_left[1]);
    let bottom_left = vec2(top_left[0], bottom_right[1]);

	let radians = rect.rotation;

    // rotate points around origin
    let top_left_rotated = rotate_point_around(top_left, pos, radians);
    let bottom_right_rotated = rotate_point_around(bottom_right, pos, radians);
    let top_right_rotated = rotate_point_around(top_right, pos, radians);
    let bottom_left_rotated = rotate_point_around(bottom_left, pos, radians);

    let top_left_pt = Point::new(top_left_rotated[0] as i32, top_left_rotated[1] as i32);
    let bottom_right_pt = Point::new(bottom_right_rotated[0] as i32, bottom_right_rotated[1] as i32);
    let top_right_pt = Point::new(top_right_rotated[0] as i32, top_right_rotated[1] as i32);
    let bottom_left_pt = Point::new(bottom_left_rotated[0] as i32, bottom_left_rotated[1] as i32);

    canvas.draw_line(top_left_pt, top_right_pt);
    canvas.draw_line(top_right_pt, bottom_right_pt);
    canvas.draw_line(bottom_right_pt, bottom_left_pt);
    canvas.draw_line(bottom_left_pt, top_left_pt);
}


impl /*FluidSimRenderer for*/ SDLFluidSimRenderer/*<'_>*/ {

    /*
    fn draw_particle(&self, particle: &Particle) {
        println!("draw_particle");
    }

    fn draw_rect(&self, rect: &Rect) {
        println!("draw_rect");
    }*/

    pub fn draw(&self, fluid_sim: &mut FluidSim, canvas: &mut WindowCanvas) {
        const draw_grid: bool = false;

        //let canvas = self.canvas;
        //let fluid_sim = self.fluid_sim;
    
        // set up scaling to render the grid to fit to window height
        let window = canvas.window();
        let (_w_width, w_height) = window.size();
        const padding: f32 = 20.0;
        const x_offset: f32 = padding;
        const y_offset: f32 = padding;
        let scale: f32 = ((w_height as f32) - (padding * 2.0)) / (fluid_sim.spatial_hash.y_size as f32);
        let offset = vec2(x_offset, y_offset);
    
        
    
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();
    
        // draw the boundary
        let width = (fluid_sim.spatial_hash.x_size as f32 * scale) as u32;
        let height = (fluid_sim.spatial_hash.y_size as f32 * scale) as u32;
        let rect = SDLRect::new(x_offset as i32, y_offset as i32, width, height);
    
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        canvas.draw_rect(rect);

        if draw_grid {
            canvas.set_draw_color(Color::RGBA(255, 0, 0, 100));
            for y in 0..fluid_sim.spatial_hash.y_size {
                for x in 0..fluid_sim.spatial_hash.x_size {
                    let x_start = (x as f32 * scale + x_offset) as i32;
                    let y_start = (y as f32 * scale + y_offset) as i32;
    
                    let w = (1.0 * scale) as u32;
                    let h = (1.0 * scale) as u32;
                    let rect = SDLRect::new(x_start, y_start, w, h);
                    canvas.draw_rect(rect);
                }
            }
        }
        
        // draw rects
        for rect in fluid_sim.rects.iter() {
            draw_rect_rotate(canvas, rect, scale, offset);
        }
            
        
        for particle in fluid_sim.particles.iter() {
            let x2 = particle.pos[0] * scale + x_offset; // simd this!
            let y2 = particle.pos[1] * scale + y_offset;
            let radius2 = 1.0 * scale;
    
            canvas.circle(x2 as i16, y2 as i16, radius2 as i16, Color::RGBA(0, 255, 0, 255));
        }
    
        canvas.present();
    }
}