use ratatui::{init, prelude::*, restore, widgets::{Block, Borders, Paragraph} };
use crate::{cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH}, sound::Chip8Sound};
use crate::input::InputHandler;
use std::time::{Duration, Instant};

const CYCLES_PER_FRAME: u8 = 10;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS);
const TARGET_FPS: u64 = 60;

pub struct Renderer {
    cpu: Chip8CPU,
    input: InputHandler,
    beeper: Chip8Sound,
    quit: bool,
}

impl Renderer {
    pub fn new(cpu: Chip8CPU, beeper: Chip8Sound) -> Self {
        Self { cpu, input: InputHandler::new(), beeper: beeper, quit: false }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = init();
        let result = self.run_loop(&mut terminal);
        restore();

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_timer_update = Instant::now();
        
        while !self.quit {
            let frame_start = Instant::now();

            for _ in 0..CYCLES_PER_FRAME {
                self.cpu.tick();
                self.beeper.update(self.cpu.sound_timer);
            }

            if last_timer_update.elapsed() >= Duration::from_micros(16667) {
                self.cpu.decrease_delay_timer();
                self.cpu.decrease_sound_timer();
                last_timer_update = Instant::now();
            }

            terminal.draw(|frame| self.draw(frame))?;
            if self.input.update_keys(&mut self.cpu.keys)? {
                self.quit = true;
            }

            let frame_time: Duration = frame_start.elapsed();
            if frame_time < FRAME_DURATION {
                std::thread::sleep(FRAME_DURATION - frame_time);
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut lines = vec![];

        for row in 0..DISPLAY_HEIGHT {
            let mut line = String::with_capacity(DISPLAY_WIDTH);
            for col in 0..DISPLAY_WIDTH {
                let byte_idx = row * 8 + (col / 8);
                let bit_idx = 7 - (col % 8);
                let pixel_on = (self.cpu.get_display()[byte_idx] >> bit_idx) & 1;

                line.push(if pixel_on == 1 {'█'} else {' '});
            }
            lines.push(Line::from(line));
        }

        let keys_status = format!("Keys: {:04X}", self.cpu.keys);

        let paragraph = Paragraph::new(lines)
            .block(Block::default()
                .title(" CHIP-8//Rust ")
                .title_bottom(format!(" ESC/DEL/Ctrl+C=exit | {} ", keys_status))
                .borders(Borders::ALL))
                .fg(Color::Green)
                .bg(Color::Black);

        frame.render_widget(paragraph, frame.area());
    }
}
