use std::fs;

struct Chip8CPU {
    memory: [u8; 4096],     // chip RAM
    stack: [u16; 16],       // stack
    display: [u8; 256],     // display
    v: [u8; 16],            // v registries
    i: u16,                 // i registry
    pc: u16,                // program counter
    sp: u8,                 // stack pointer
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Chip8CPU {
    fn new() -> Self {
        let mut cpu: Chip8CPU = Chip8CPU { 
            memory: [0; 4096], 
            stack: [0; 16],
            display: [0; 256],
            v: [0; 16], 
            i: 0, pc: 0, sp: 0 
        };
        cpu.pc = 0x200;
        cpu.memory[0..80].copy_from_slice(&FONT);
        cpu
    }

    fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let rom = fs::read(path).map_err(|e| format!("Failed to read ROM: {}", e))?;

        if rom.len() > 4096 - 0x200 { // all avaliable space on chip
            return Err(format!("ROM is too big! ({})", rom.len()));
        }
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);

        Ok(())
    }
}

fn main() {
    let _cpu = Chip8CPU::new();

    let quit: bool = false;
    while quit {
        
    }
}
