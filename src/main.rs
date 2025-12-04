mod cpu;

fn main() {
    let mut cpu = cpu::Chip8CPU::new();

    cpu.load_rom("test.ch8").expect("No file.ch8 :(");

    let quit: bool = false;
    while !quit {
        cpu.cycle();
    }
}