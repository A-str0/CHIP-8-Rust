struct Chip8CPU {
    memory: [u8; 4096],     // chip memory
    v: [u8; 16],            // v registries
    i: u16,                 // i registry
}

fn main() {
    let cpu: Chip8CPU = Chip8CPU { memory: [0; 4096], v: [0; 16], i: 0 };

    println!("{} | {} | {}", cpu.memory[0], cpu.v[0], cpu.i);
}
