use std::fs;

struct Chip8CPU {
    memory: [u8; 4096],     // chip RAM
    stack: [u16; 16],       // stack
    display: [u8; 256],     // display
    v: [u8; 16],            // v registries
    i: u16,                 // i registry
    pc: u16,                // program counter
    sp: u8,                 // stack pointer

    delay_timer: u8,
    sound_timer: u8,
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
const WORD: u16 = 2;

impl Chip8CPU {
    fn new() -> Self {
        let mut cpu: Chip8CPU = Chip8CPU { 
            memory: [0; 4096], 
            stack: [0; 16],
            display: [0; 256],
            v: [0; 16], 
            i: 0, pc: 0, sp: 0,
            delay_timer: 0,
            sound_timer: 0,
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

    fn cycle(&mut self) {
        let oppcode = self.fetch_oppcode();

        self.execute(oppcode);
    }

    fn fetch_oppcode(&self) -> u16 {
        todo!()
    }

    fn execute(&mut self, opcode: u16) -> Result<(), String> {
        let nnn = opcode & 0x0FFF;

        // oppcode is 16 bytes so it can be separeted into 4 nibbles (полубайт, если по-русски)
        // in our case nibble is type with four nibbles (4 bits)
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),               // CLS | clear display
            (0x0, 0x0, 0xE, 0xE) => self.ret(),               // RET | retuern to the oppcode of stack
            (0x0, _, _, _)       => self.sys(nnn),      // SYS | jump to machine code routine
            (0x1, _, _, _)       => self.jp(nnn),       // JP | jump to location
            (0x2, _, _, _)       => self.call(nnn),     //  CALL | call a subroutine
            _ => {
                return Err(format!("oppcode is not recognized! ({})", opcode));
            }
        }

        return Ok(());
    }

    fn sys(&mut self, addr: u16) {
        println!("OUTDATED!!! not in use anymore :)");
    }

    fn cls(&mut self) {
        self.display.iter_mut().for_each(|pixel| { *pixel = 0; });
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += WORD;
    }

    fn jp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }
}

fn main() {
    let _cpu = Chip8CPU::new();

    let quit: bool = false;
    while quit {
        
    }
}
