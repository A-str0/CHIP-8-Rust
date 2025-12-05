use sdl2::{render::Canvas, video::Window};

use crate::cpu::{Chip8CPU, DISPLAY_SIZE_X, DISPLAY_SIZE_Y};

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 example", 800, 600)
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_logical_size(DISPLAY_SIZE_X as u32, DISPLAY_SIZE_Y as u32)
            .map_err(|e| e.to_string())?;

        let s =  Self { 
            canvas: canvas,
        };

        Ok(s)
    }

    pub fn cycle(&mut self, _cpu: &mut Chip8CPU) {
        println!("asdasdas");
    }
}