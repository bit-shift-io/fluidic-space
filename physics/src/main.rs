#![feature(portable_simd)]
#![feature(stmt_expr_attributes)]
#![feature(generic_associated_types)]
#![feature(test)]

use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::video::WindowPos;
use std::time::Duration;

use core_simd::*;

use libphysics::*;

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

fn render(canvas: &mut WindowCanvas, fluid_sim: &mut FluidSim) {

    const draw_grid: bool = false;

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
    let rect = Rect::new(x_offset as i32, y_offset as i32, width, height);

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
                let rect = Rect::new(x_start, y_start, w, h);
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

fn update(fluid_sim: &mut FluidSim) {
    fluid_sim.update(0.001);
}


fn main() -> Result<(), String> {
    //basic_fluid::init_world();
    libphysics::test();

    const GRID_SIZE: usize = 100;
    const PARTICLE_COUNT: usize = 400;
    const SLEEP_PER_FRAME_MS: u64 = 0;

    let mut fluid_sim = FluidSim::new(GRID_SIZE, GRID_SIZE);
    //fluid_sim.collision_energy_loss = 0.5;
    //fluid_sim.elasticity = 20.0;
    fluid_sim.properties.damping = 1.0; //0.999; // might want a contact gamping and non-contact damping?
    fluid_sim.properties.collision_damping = 1.0;
    // so we want a high velocity when in contact really close, but as we mov out the velocity is dampened/drained
    // and the push away force also grows less, this *should* maybe help particle push out without having such extreme
    // velocities once they 'disconnect'
    fluid_sim.properties.gravity = vec2(0.0, 98.0);

    /*
    fluid_sim.shapes.push(
        Box::new(fluid_sim::Rect {
            pos: Simd::from_array([50.0, 50.0]),
            size: Simd::from_array([10.0, 10.0]),
        })
    );*/
    fluid_sim.rects.push(
        libphysics::Rect {
            pos: Simd::from_array([30.0, 50.0]),
            size: Simd::from_array([30.0, 10.0]),
            rotation: (20.0 as f32).to_radians()
        }
    );

    fluid_sim.rects.push(
        libphysics::Rect {
            pos: Simd::from_array([70.0, 50.0]),
            size: Simd::from_array([30.0, 10.0]),
            rotation: (-20.0 as f32).to_radians()
        }
    );

    let particles = fluid_sim.generate_random_particles(PARTICLE_COUNT);

    //let mut particles = vec![];
    //particles.push(Particle::with_vel(vec2(45.0, 30.0), vec2(0.0, 0.0)));

    fluid_sim.add_particles(&particles);

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
        ::std::thread::sleep(Duration::from_millis(SLEEP_PER_FRAME_MS));
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