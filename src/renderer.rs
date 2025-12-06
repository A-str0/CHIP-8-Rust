use ratatui::{
    init, prelude::*, restore, widgets::{Block, Borders, Paragraph}
};
use crate::cpu::{Chip8CPU, DISPLAY_HEIGHT, DISPLAY_WIDTH};

const TICK_PER_CYCLE: u8 = 12;

pub struct Renderer {
    cpu: Chip8CPU,
    quit: bool,
}

impl Renderer {
    pub fn new(cpu: Chip8CPU) -> Self {
        Self { cpu, quit: false }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut terminal = init();
        let result = self.run_loop(&mut terminal);
        restore();

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
        while !self.quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            for _ in 0..TICK_PER_CYCLE {
                self.cpu.tick();
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut lines = vec![];

        for row in 0..DISPLAY_WIDTH {
            let mut line = String::with_capacity(DISPLAY_HEIGHT);
            for col in 0..DISPLAY_HEIGHT {
                let byte_idx = row * 8 + (col / 8);
                let bit_idx = 7 - (col % 8);
                let pixel_on = (self.cpu.get_display()[byte_idx] >> bit_idx) & 1;

                line.push(if pixel_on == 1 {'█'} else {' '});
            }
            lines.push(Line::from(line));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().title(" CHIP-8//Rust ")
            .borders(Borders::ALL))
            .fg(Color::Green)
            .bg(Color::Black);

        frame.render_widget(paragraph, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};

        if poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => self.quit = true,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}