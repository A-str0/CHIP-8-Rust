use ratatui::{ crossterm::{event::{Event, KeyCode, poll, read}, terminal::enable_raw_mode}, init, prelude::*, restore, widgets::{Block, Borders, Paragraph} };
use crate::cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use std::{time::{Duration, Instant}};

const INPUT_BUFFER: u8 = 4;
const CYCLES_PER_FRAME: u8 = 10;
const TARGET_FPS: u64 = 60;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS);

pub struct Renderer {
    cpu: Chip8CPU,
    quit: bool,
    input: u16,
    cur_input: [u8; 16],
}

impl Renderer {
    pub fn new(cpu: Chip8CPU) -> Self {
        Self { cpu, quit: false, input: 0, cur_input: [0; 16] }
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
            
            self.handle_events()?;
            for i in 0..16 {
                if self.cur_input[i] <= 0 || self.cur_input[i] >= INPUT_BUFFER {
                    continue;
                }

                if ((self.input >> i) & 1) != 0 {
                    self.cur_input[i] += 1;
                } else {
                    self.cur_input[i] -= 1;
                }
            }
            self.input = 0;

            for _ in 0..CYCLES_PER_FRAME {
                self.handle_events()?;
                for i in 0..16 {
                    if self.cur_input[i] > 0 {
                        self.cpu.keys |= 1 << i;
                    }
                }

                self.cpu.tick();
            }

            if last_timer_update.elapsed() >= Duration::from_micros(16667) {
                self.cpu.decrease_delay_timer();
                self.cpu.decrease_sound_timer();
                last_timer_update = Instant::now();
            }

            terminal.draw(|frame| self.draw(frame))?;

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
                .title_bottom(format!(" DEL=exit | {} ", keys_status))
                .borders(Borders::ALL))
            .fg(Color::Green)
            .bg(Color::Black);

        frame.render_widget(paragraph, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        while poll(Duration::ZERO)? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Delete                         => self.quit = true,

                    KeyCode::Char('1')                      => self.input |= 1 << 0x1,
                    KeyCode::Char('2')                      => self.input |= 1 << 0x2,
                    KeyCode::Char('3')                      => self.input |= 1 << 0x3,
                    KeyCode::Char('4')                      => self.input |= 1 << 0xC,
                    KeyCode::Char('q') | KeyCode::Char('Q') => self.input |= 1 << 0x4,
                    KeyCode::Char('w') | KeyCode::Char('W') => self.input |= 1 << 0x5,
                    KeyCode::Char('e') | KeyCode::Char('E') => self.input |= 1 << 0x6,
                    KeyCode::Char('r') | KeyCode::Char('R') => self.input |= 1 << 0xD,
                    KeyCode::Char('a') | KeyCode::Char('A') => self.input |= 1 << 0x7,
                    KeyCode::Char('s') | KeyCode::Char('S') => self.input |= 1 << 0x8,
                    KeyCode::Char('d') | KeyCode::Char('D') => self.input |= 1 << 0x9,
                    KeyCode::Char('f') | KeyCode::Char('F') => self.input |= 1 << 0xE,
                    KeyCode::Char('z') | KeyCode::Char('Z') => self.input |= 1 << 0xA,
                    KeyCode::Char('x') | KeyCode::Char('X') => self.input |= 1 << 0x0,
                    KeyCode::Char('c') | KeyCode::Char('C') => self.input |= 1 << 0xB,
                    KeyCode::Char('v') | KeyCode::Char('V') => self.input |= 1 << 0xF,

                    _ => {},
                }
            }
        }
        Ok(())
    }
}