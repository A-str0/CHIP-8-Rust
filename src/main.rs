mod cpu;
mod input;
mod renderer;

fn main() -> Result<(), String> {
    let mut cpu = cpu::Chip8CPU::new()?;

    cpu.reset()?;
    cpu.load_rom("Brick.ch8")?;

    let mut renderer = renderer::Renderer::new(cpu);
    renderer.run().map_err(|e| format!("Render error: {}", e))?;

    println!("Emulator stopped. Bye! :)");
    Ok(())
}