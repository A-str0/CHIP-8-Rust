use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH};

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
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        canvas.set_logical_size(DISPLAY_HEIGHT as u32, DISPLAY_WIDTH as u32)
            .map_err(|e| e.to_string())?;

        let s =  Self { 
            canvas: canvas,
        };

        Ok(s)
    }

    pub fn draw(&mut self, cpu: &Chip8CPU) -> Result<(), String> {
        // Чёрный фон
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Зелёный пиксель (можно любой цвет)
        self.canvas.set_draw_color(Color::RGB(0, 255, 100));

        // Проходим по всей битовой карте (256 байт = 2048 пикселей)
        for (byte_idx, &byte) in cpu.get_display().iter().enumerate() {
            // Координаты строки и базовой колонки
            let y = byte_idx / 8;                     // 256 байт → 32 строки
            let x_base = (byte_idx % 8) * 8;           // каждый байт → 8 пикселей по X

            // Проверяем каждый бит в байте
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