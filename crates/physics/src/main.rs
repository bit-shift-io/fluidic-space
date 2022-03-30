#![feature(portable_simd)]

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;

use crate::spatial_hash::*;

mod third_party;
//mod basic_fluid;
mod simd_test;

mod spatial_hash;

fn render(canvas: &mut WindowCanvas, fluid_sim: &mut SpatialHash) {
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();

    const x_offset: f32 = 50.0;
    const y_offset: f32 = 50.0;

    const scale: f32 = 50.0;

    // draw the boundary
    let width = (fluid_sim.x_size as f32 * scale) as u32;
    let height = (fluid_sim.y_size as f32 * scale) as u32;
    let rect = Rect::new(x_offset as i32, y_offset as i32, width, height);

    canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    canvas.draw_rect(rect);

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

    // https://rust-sdl2.github.io/rust-sdl2/sdl2/render/struct.Canvas.html#method.circle
    //canvas.circle(16, 16, 16, Color::RGBA(0, 0, 0, 255));

    let render_c = |x: f32, y: f32| {
        // scale up to a visible range
        // this part could be simd accelerated?
        let x2 = x * scale + x_offset;
        let y2 = y * scale + y_offset;
        let radius = 1.0 * scale;
        canvas.circle(x2 as i16, y2 as i16, radius as i16, Color::RGBA(0, 255, 0, 255));
    };
    fluid_sim.for_each_pos(render_c);

    canvas.present();
}

fn update(fluid_sim: &mut SpatialHash) {
    fluid_sim.update_velocity_from_collisions();
    fluid_sim.apply_velocity(0.1);

    fluid_sim.swap();
    fluid_sim.clear_next();

    println!("updated");
}

fn main() -> Result<(), String> {
    //third_party::third_party_test();
    //basic_fluid::init_world();
    //simd_test::simd_test();

    let mut fluid_sim = SpatialHash::new(8, 8, 4 * 2);
    //let mut pts = fluid_sim.generate_random_points(5);
    let mut pts = vec![1.0, 1.0, 1.8, 1.8];
    fluid_sim.add_points(&pts);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

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
        ::std::thread::sleep(Duration::from_millis(1000));
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