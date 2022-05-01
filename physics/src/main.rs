#![feature(portable_simd)]
#![feature(stmt_expr_attributes)]
#![feature(generic_associated_types)]
#![feature(test)]
#![feature(nll)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use core_simd::*;

use libphysics::*;
use libphysicsrender::*;

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

    let mut sdl = SdlSystem::new("Fluidic Space - Fluid Dynamics", 800, 600);
    let fluid_sim_renderer = SdlFluidSimRenderer::new(&mut fluid_sim, &mut sdl.canvas);

    let mut event_pump = sdl.sdl_context.event_pump()?;
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
        fluid_sim.update(0.001);

        // Render
        fluid_sim_renderer.draw();

        // Time management!
        //Duration::from_millis(1000)
        ::std::thread::sleep(Duration::from_millis(SLEEP_PER_FRAME_MS));
        //::std::thread::sleep(Duration::from::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
