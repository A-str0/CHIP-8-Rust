use crate::sound::Chip8Sound;
use ratatui::{init, prelude::*, restore};
use std::time::{Duration, Instant};

mod cpu;
mod input;
mod renderer;
mod sound;

const CYCLES_PER_FRAME: u8 = 10;
const FRAME_DURATION: Duration = Duration::from_micros(1_000_000 / TARGET_FPS);
const TARGET_FPS: u64 = 60;

fn main() -> Result<(), String> {
    let mut cpu = cpu::Chip8CPU::new()?;
    let mut beeper = Chip8Sound::new();

    cpu.reset()?;
    cpu.load_rom("Brick.ch8")?;

    let mut renderer = renderer::Renderer::new();
    let mut terminal = init();
    let result = run_loop(&mut terminal, &mut cpu, &mut beeper, &mut renderer);
    restore();

    result.map_err(|e| format!("Render error: {}", e))?;

    println!("Emulator stopped. Bye! :)");
    Ok(())
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    cpu: &mut cpu::Chip8CPU,
    beeper: &mut Chip8Sound,
    renderer: &mut renderer::Renderer,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_timer_update = Instant::now();
    let mut quit = false;

    while !quit {
        let frame_start = Instant::now();

        for _ in 0..CYCLES_PER_FRAME {
            cpu.tick();
            beeper.update(cpu.sound_timer);
        }

        if last_timer_update.elapsed() >= Duration::from_micros(16667) {
            cpu.decrease_delay_timer();
            cpu.decrease_sound_timer();
            last_timer_update = Instant::now();
        }

        terminal.draw(|frame| renderer.draw(frame, cpu))?;
        quit = renderer.should_quit(&mut cpu.keys)?;

        let frame_time = frame_start.elapsed();
        if frame_time < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_time);
        }
    }
    Ok(())
}
