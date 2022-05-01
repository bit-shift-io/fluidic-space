use sdl2::rect::Point;
use sdl2::rect::Rect as SDLRect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;

use libphysics::FluidSim;
use libphysics::*;

pub struct SdlFluidSimRenderer/*<'a>*/ {
    fluid_sim: *mut FluidSim,
    //fluid_sim: &'a mut FluidSim,
    canvas: *mut WindowCanvas
}

impl SdlFluidSimRenderer/*<'_>*/ {
    pub fn new/*<'a>*/(fluid_sim: *mut FluidSim, canvas: *mut WindowCanvas) -> SdlFluidSimRenderer/*<'a>*/ {
        SdlFluidSimRenderer {
            fluid_sim,
            //fluid_sim,
            canvas
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

    canvas.draw_line(top_left_pt, top_right_pt).ok();
    canvas.draw_line(top_right_pt, bottom_right_pt).ok();
    canvas.draw_line(bottom_right_pt, bottom_left_pt).ok();
    canvas.draw_line(bottom_left_pt, top_left_pt).ok();
}


impl /*FluidSimRenderer for*/ SdlFluidSimRenderer/*<'_>*/ {

    /*
    fn draw_particle(&self, particle: &Particle) {
        println!("draw_particle");
    }

    fn draw_rect(&self, rect: &Rect) {
        println!("draw_rect");
    }*/

    pub fn draw(&self) {
        unsafe {
            let draw_grid: bool = false;

            let canvas: &mut WindowCanvas = &mut *self.canvas;
            let fluid_sim: &mut FluidSim = &mut *self.fluid_sim;
        
            // set up scaling to render the grid to fit to window height
            let window = canvas.window();
            let (_w_width, w_height) = window.size();
            let padding: f32 = 20.0;
            let x_offset: f32 = padding;
            let y_offset: f32 = padding;
            let scale: f32 = ((w_height as f32) - (padding * 2.0)) / (fluid_sim.spatial_hash.y_size as f32);
            let offset = vec2(x_offset, y_offset);
        
            
        
            canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
            canvas.clear();
        
            // draw the boundary
            let width = (fluid_sim.spatial_hash.x_size as f32 * scale) as u32;
            let height = (fluid_sim.spatial_hash.y_size as f32 * scale) as u32;
            let rect = SDLRect::new(x_offset as i32, y_offset as i32, width, height);
        
            canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            canvas.draw_rect(rect).ok();

            if draw_grid {
                canvas.set_draw_color(Color::RGBA(255, 0, 0, 100));
                for y in 0..fluid_sim.spatial_hash.y_size {
                    for x in 0..fluid_sim.spatial_hash.x_size {
                        let x_start = (x as f32 * scale + x_offset) as i32;
                        let y_start = (y as f32 * scale + y_offset) as i32;
        
                        let w = (1.0 * scale) as u32;
                        let h = (1.0 * scale) as u32;
                        let rect = SDLRect::new(x_start, y_start, w, h);
                        canvas.draw_rect(rect).ok();
                    }
                }
            }
            
            // draw rects
            for rect in fluid_sim.rects.iter() {
                draw_rect_rotate(canvas, rect, scale, offset);
            }
                
            // https://john-wigg.dev/2DMetaballs/
            //let mut edge_particles: Vec<*const Particle> = Vec::new();
            for particle in fluid_sim.particles.iter() {
                /*
                let is_edge = particle.contacts.len() <= 1;
                if is_edge {
                    edge_particles.push(&*particle);
                }*/

                let x2 = particle.pos[0] * scale + x_offset; // simd this!
                let y2 = particle.pos[1] * scale + y_offset;
                let radius2 = 1.0 * scale;
        
                let color = Color::RGBA(0, 255, 0, 255); //if is_edge { Color::RGBA(0, 0, 255, 255) } else { Color::RGBA(0, 255, 0, 255) };
                canvas.circle(x2 as i16, y2 as i16, radius2 as i16, color).ok();
            }
        
            canvas.present();
        }
    }
}