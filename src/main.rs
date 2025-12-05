use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

mod cpu;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cpu = cpu::Chip8CPU::new();
    let mut ren = renderer::Renderer::new()?;

    cpu.load_rom("Brick.ch8").expect("No Brick.ch8 :(");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nCtrl-C received, shutting down gracefully...");
    })?;

    while running.load(Ordering::SeqCst) {
        // Your main program logic here
        cpu.cycle();
        ren.cycle(&mut cpu);
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("Program terminated.");
    Ok(())
}