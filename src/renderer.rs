use sdl2::{EventPump, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct Renderer {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

impl Renderer {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 example", 800, 600)
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_logical_size(DISPLAY_HEIGHT as u32, DISPLAY_WIDTH as u32)
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        let s =  Self { 
            canvas: canvas,
            event_pump: event_pump,
        };

        Ok(s)
    }

    pub fn handle_input(&mut self, cpu: &mut Chip8CPU) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,

                sdl2::event::Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(chip8_key) = Renderer::sdl_to_chip8(key) {  // ← теперь &self!
                        cpu.keys |= 1 << chip8_key;
                    }
                }

                sdl2::event::Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(chip8_key) = Renderer::sdl_to_chip8(key) {
                        cpu.keys &= !(1 << chip8_key);
                    }
                }

                _ => {}
            }
        }
        true
    }

    fn sdl_to_chip8(key: Keycode) -> Option<u8> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }

    pub fn draw(&mut self, cpu: &Chip8CPU) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 255, 100));

        for (byte_idx, &byte) in cpu.get_display().iter().enumerate() {
            let y = byte_idx / 8;
            let x_base = (byte_idx % 8) * 8;

            for bit in 0..8 {
                if (byte & (1 << (7 - bit))) != 0 {
                    let pixel_x = x_base + bit;
                    let pixel_y = y;

                    let rect = Rect::new(
                        pixel_x as i32,
                        pixel_y as i32,
                        1,
                        1,
                    );
                    self.canvas.fill_rect(rect)?;
                }
            }
        }

        self.canvas.present();
        Ok(())
    }
}