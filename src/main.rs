use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::{Duration, Instant}};

mod cpu;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = cpu::Chip8CPU::new();
    let mut renderer = renderer::Renderer::new()?;

    cpu.load_rom("Brick.ch8")?;
    // cpu.load_rom("tests/1-chip8-logo.ch8")?;
    // cpu.load_rom("tests/2-ibm-logo.ch8")?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nCtrl+C received, shutting down...");
    })?;

    const CPU_CYCLES_PER_FRAME: u32 = 12;
    const FRAME_DURATION: Duration = Duration::from_nanos(16_666_666);
    const TIMER_DURATION: Duration = Duration::from_nanos(16_666_666);

    let mut last_frame = Instant::now();
    let mut last_timer = Instant::now();

    while running.load(Ordering::SeqCst) {
        let now = Instant::now();

        for _ in 0..CPU_CYCLES_PER_FRAME {
            cpu.cycle();
        }

        if now - last_timer >= TIMER_DURATION {
            cpu.decrease_delay_timer();
            cpu.decrease_sound_timer();
            last_timer = now;
        }

        renderer.draw(&mut cpu)?;

        let elapsed = now.duration_since(last_frame);
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
        last_frame = Instant::now();
    }

    println!("Emulator stopped. Bye! :)");
    Ok(())
}