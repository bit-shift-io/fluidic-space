use sdl2::Sdl;
use sdl2::video::WindowPos;
use sdl2::render::WindowCanvas;

pub struct SdlSystem {
    pub sdl_context: Sdl,
    //window: Window,
    pub canvas: WindowCanvas
}

impl SdlSystem {
    pub fn new(title: &str, width: u32, height: u32) -> SdlSystem {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem.window(title, width, height)
            //.position_centered()
            .build()
            .expect("could not initialize video subsystem");
        window.set_position(WindowPos::Positioned(0), WindowPos::Positioned(0)); // easy to debug

        let canvas = window.into_canvas().build()
            .expect("could not make a canvas");

        SdlSystem {
            sdl_context,
            //window,
            canvas
        }
    }
/*
    pub fn run_event_loop<F: Fn(f32)>(&mut self, update: F) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
/*

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
            fluid_sim.update(0.001);

            // Render
            fluid_sim_renderer.draw();

            // Time management!
            //Duration::from_millis(1000)
            ::std::thread::sleep(Duration::from_millis(SLEEP_PER_FRAME_MS));
            //::std::thread::sleep(Duration::from::new(0, 1_000_000_000u32 / 60));
        }*/
    } */
}