use ratatui::{prelude::*, widgets::{Block, Borders, Paragraph} };
use crate::{cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH}};
use crate::input::InputHandler;

pub struct Renderer {
    input: InputHandler,
}

impl Renderer {
    pub fn new() -> Self {
        Self { input: InputHandler::new() }
    }

    pub fn draw(&mut self, frame: &mut Frame, cpu: &Chip8CPU) {
        let mut lines = vec![];

        for row in 0..DISPLAY_HEIGHT {
            let mut line = String::with_capacity(DISPLAY_WIDTH);
            for col in 0..DISPLAY_WIDTH {
                let byte_idx = row * 8 + (col / 8);
                let bit_idx = 7 - (col % 8);
                let pixel_on = (cpu.get_display()[byte_idx] >> bit_idx) & 1;

                line.push(if pixel_on == 1 {'▒'} else {' '});
            }
            lines.push(Line::from(line));
        }

        let keys_status = format!("Keys: {:04X}", cpu.keys);

        let paragraph = Paragraph::new(lines)
            .block(Block::default()
                .title(" CHIP-8//Rust ")
                .title_bottom(format!(" press ESC/DEL to exit | {} ", keys_status))
                .borders(Borders::ALL))
                .fg(Color::Green)
                .bg(Color::Black);

        frame.render_widget(paragraph, frame.area());
    }

    pub fn should_quit(&mut self, keys: &mut u16) -> Result<bool, Box<dyn std::error::Error>> {
        self.input.update_keys(keys)
    }


}
