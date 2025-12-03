use std::{fs, u8};

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
const DISPLAY_SIZE_X: usize = 64;
const DISPLAY_SIZE_Y: usize = 32;


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

        self.execute(oppcode).expect("error lol");
    }

    fn fetch_oppcode(&mut self) -> u16 {
        let high = self.memory[self.pc as usize] as u16;
        let low  = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += WORD;
        (high << 8) | low
    }

    fn execute(&mut self, opcode: u16) -> Result<(), String> {
        let nnn   = opcode & 0x0FFF;
        let x   = ((opcode & 0x0F00) >> 8) as usize;
        let y   = ((opcode & 0x00F0) >> 4) as usize;
        let kk: u8     = (opcode & 0x00FF) as u8;

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
            (0x2, _, _, _)       => self.call(nnn),     // CALL | call a subroutine
            (0x3, _, _, _)       => self.se_kk(x, kk),        // SE | skip next instruction if V[x] == kk
            (0x4, _, _, _)       => self.sne_kk(x, kk),          // SNE | skip next instruction if V[x] != kk
            (0x5, _, _, 0x0)     => self.se_y(x, y),          // SE | skip next instruction if V[x] == V[y]
            (0x6, _, _, _)       => self.ld_ll(x, kk),           // LD | set V[x] = kk
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
    fn jp_v0(&mut self, addr: u16) { 
        self.pc = addr + self.v[0x0] as u16;
    }

    fn call(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn se_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += WORD;
        }
    }
    fn se_y(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += WORD;
        }
    }
    
    fn sne_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += WORD;
        }
    }
    fn sne_y(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += WORD;
        }
    }

    fn ld_ll(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }
    fn ld_y(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }
    fn ld_i(&mut self, addr: u16) {
        self.i = addr;
    }

    fn add_kk(&mut self, x: usize, kk: u8) {
        let (res, overflow) = self.v[x].overflowing_add(kk);
        self.v[0xF] = (overflow) as u8;
        self.v[x] = res;
    }
    fn add_y(&mut self, x: usize, y: usize) {
        self.add_kk(x, self.v[y]);
    }

    fn sub(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = res;
        self.v[0xF]= overflow as u8;
    }
    fn subn(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = res;
        self.v[0xF]= overflow as u8;
    }

    fn or(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn and(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn xor(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn shr(&mut self, x: usize, y: usize) {
        let vx = self.v[x];
        self.v[0xF] = vx & 1;
        self.v[x] = vx >> 1;
    }

    fn shl(&mut self, x: usize, y: usize) {
        let vx = self.v[x];
        self.v[0xF] = (vx >> 7) & 1;
        self.v[x] = vx << 1;
    }

    fn rnd(&mut self, x: usize, kk: u8) {
        // TODO
    }

    fn drw(&mut self, x: usize, y: usize, n: u8) {
        let sprite_addr = self.pc as usize;
        let pos_x = (self.v[x] as usize) % DISPLAY_SIZE_X;
        let pos_y = (self.v[y] as usize) % DISPLAY_SIZE_Y;

        self.v[0xF] = 0;

        for row in 0..n as usize {
            let sprite_byte = self.memory[sprite_addr + row];

            for bit in 0..8 {
                let pixel_x = (pos_x + bit) % DISPLAY_SIZE_X;
                let pixel_y = (pos_y + row) % DISPLAY_SIZE_Y;

                let idx = pixel_y * DISPLAY_SIZE_X + pixel_x;

                let sprite_pixel = (sprite_byte >> (7 - bit)) & 1;
                let screen_pixel = self.display[idx];
                let new_pixel = screen_pixel ^ sprite_pixel;
                if screen_pixel == 1 && new_pixel == 0 {
                    self.v[0xF] = 1;
                }

                self.display[idx] = new_pixel;
            }
        }
    }

}

fn main() {
    let mut cpu = Chip8CPU::new();

    let quit: bool = false;
    while quit {
        cpu.cycle();
    }
}
