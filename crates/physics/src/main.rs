#![feature(portable_simd)]
#![feature(stmt_expr_attributes)]

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::WindowPos;
use std::time::Duration;

use crate::fluid_sim::*;
use core_simd::*;

mod third_party;
//mod basic_fluid;
mod simd_test;

mod fluid_sim;

fn render(canvas: &mut WindowCanvas, fluid_sim: &mut FluidSim) {

    const draw_grid: bool = false;

    // set up scaling to render the grid to fit to window height
    let window = canvas.window();
    let (_w_width, w_height) = window.size();
    const padding: f32 = 20.0;
    const x_offset: f32 = padding;
    const y_offset: f32 = padding;
    let scale: f32 = ((w_height as f32) - (padding * 2.0)) / (fluid_sim.y_size as f32);



    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();

    // draw the boundary
    let width = (fluid_sim.x_size as f32 * scale) as u32;
    let height = (fluid_sim.y_size as f32 * scale) as u32;
    let rect = Rect::new(x_offset as i32, y_offset as i32, width, height);

    canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    canvas.draw_rect(rect);

    if draw_grid {
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 100));
        for y in 0..fluid_sim.y_size {
            for x in 0..fluid_sim.x_size {
                let x_start = (x as f32 * scale + x_offset) as i32;
                let y_start = (y as f32 * scale + y_offset) as i32;

                let w = (1.0 * scale) as u32;
                let h = (1.0 * scale) as u32;
                let rect = Rect::new(x_start, y_start, w, h);
                canvas.draw_rect(rect);
            }
        }
    }

    // https://rust-sdl2.github.io/rust-sdl2/sdl2/render/struct.Canvas.html#method.circle
    //canvas.circle(16, 16, 16, Color::RGBA(0, 0, 0, 255));

    // THIS IS HORRIBLY SLOW! rethink how we do this
    let render_c = #[inline(always)] |pt: f32x2| {
        // scale up to a visible range
        // this part could be simd accelerated?
        let x2 = pt[0] * scale + x_offset;
        let y2 = pt[1] * scale + y_offset;
        let radius = 1.0 * scale;
        canvas.circle(x2 as i16, y2 as i16, radius as i16, Color::RGBA(0, 255, 0, 255));
    };
    fluid_sim.for_each_pos(render_c);

    canvas.present();
}

fn update(fluid_sim: &mut FluidSim) {
    const gravity: f32x2 = Simd::from_array([0.0, 0.1]);

    fluid_sim.update_velocity_from_collisions();
    fluid_sim.add_uniform_velocity(gravity); // some gravity
    fluid_sim.apply_velocity(0.01);
    fluid_sim.swap();
    fluid_sim.clear_next_simd(false);

    //println!("updated");
}

fn main() -> Result<(), String> {
    //third_party::third_party_test();
    //basic_fluid::init_world();
    simd_test::simd_test();

    const grid_size: usize = 100;
    const particle_count: usize = 400;
    const max_particles_per_cell: usize = 2;
    const sleep_per_frame_ms: u64 = 0;

    let mut fluid_sim = FluidSim::new(grid_size, grid_size, max_particles_per_cell);
    //fluid_sim.collision_energy_loss = 0.5;
    fluid_sim.elasticity = 0.5;
    fluid_sim.damping = 0.99; //0.999;

    let mut pts = fluid_sim.generate_random_points(particle_count);
    //let mut pts = vec![1.0, 1.0, 1.8, 1.8];
    //let mut pts = vec![1.5, 1.5, 3.3, 1.5];
    //let mut pts = vec![2.5, 2.5, 1.5, 3.5];
    fluid_sim.add_points(&pts);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem.window("Fluidic Space - Fluid Dynamics", 800, 600)
        //.position_centered()
        .build()
        .expect("could not initialize video subsystem");
    window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0)); // easy to debug

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Update
        update(&mut fluid_sim);

        // Render
        render(&mut canvas, &mut fluid_sim);

        // Time management!
        //Duration::from_millis(1000)
        ::std::thread::sleep(Duration::from_millis(sleep_per_frame_ms));
        //::std::thread::sleep(Duration::from::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

/*


use libphysics;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    let somevalue = 123;
    libphysics::public_function();
    println!("Hello, world! {}", somevalue);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
*/