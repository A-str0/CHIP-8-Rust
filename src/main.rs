use crate::sound::Chip8Sound;

mod cpu;
mod input;
mod renderer;
mod sound;

fn main() -> Result<(), String> {
    let mut cpu = cpu::Chip8CPU::new()?;
    let bpr = Chip8Sound::new();

    cpu.reset()?;
    cpu.load_rom("Brick.ch8")?;

    let mut renderer = renderer::Renderer::new(cpu, bpr);
    renderer.run().map_err(|e| format!("Render error: {}", e))?;

    println!("Emulator stopped. Bye! :)");
    Ok(())
}