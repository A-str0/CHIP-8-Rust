use std::fs;

const FONTS: [u8; 80] = [
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
pub const DISPLAY_HEIGHT: usize = 64;
pub const DISPLAY_WIDTH: usize  = 32;
const WORD: u16 = 2;
pub struct Chip8CPU {
    memory: [u8; 4096],     // chip RAM
    stack: [u16; 16],       // stack
    display: [u8; (DISPLAY_HEIGHT * DISPLAY_WIDTH) / 8],     // display
    v: [u8; 16],            // v registries
    i: u16,                 // i registry
    pc: u16,                // program counter
    sp: u8,                 // stack pointer

    delay_timer: u8,
    sound_timer: u8,
    // TODO: change to bitmask
    keys: [bool; 16],       // all keys
}

impl Chip8CPU {
    pub fn get_display(&self) -> &[u8; (DISPLAY_HEIGHT * DISPLAY_WIDTH) / 8] { &self.display }

    pub fn decrease_delay_timer(&mut self) {
        if self.delay_timer > 0 { 
            self.delay_timer -= 1; 
        }
    }
    pub fn decrease_sound_timer(&mut self) {
        if self.sound_timer > 0 { 
            self.sound_timer -= 1; 
        }
    }

    pub fn new() -> Self {
        let mut cpu: Chip8CPU = Chip8CPU { 
            memory: [0; 4096], 
            stack: [0; 16],
            display: [0; (DISPLAY_HEIGHT * DISPLAY_WIDTH) / 8],
            v: [0; 16], 
            i: 0, pc: 0, sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
        };
        cpu.pc = 0x200;
        cpu.memory[0..80].copy_from_slice(&FONTS);
        cpu
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let rom = fs::read(path).map_err(|e| format!("Failed to read ROM: {}", e))?;

        if rom.len() > 4096 - 0x200 { // all avaliable space on chip
            return Err(format!("ROM is too big! ({})", rom.len()));
        }
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);

        Ok(())
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch_oppcode();

        println!("OPCODE: {:04X} (PC: {})", opcode, self.pc);

        self.execute(opcode).expect("error lol");
    }

    fn fetch_oppcode(&mut self) -> u16 {
        if self.pc >= 4096 - 1 { return 0; }

        let high = self.memory[self.pc as usize] as u16;
        let low  = self.memory[(self.pc + 1) as usize] as u16;
        (high << 8) | low
    }

    fn execute(&mut self, opcode: u16) -> Result<(), String> {
        let nnn   = opcode & 0x0FFF;
        let x   = ((opcode & 0x0F00) >> 8) as usize;
        let y   = ((opcode & 0x00F0) >> 4) as usize;
        let kk: u8     = (opcode & 0x00FF) as u8;
        let n      = (opcode & 0x000F) as u8;

        // oppcode is 16 bytes so it can be separeted into 4 nibbles (полубайт, если по-русски)
        // in our case nibble is type with four nibbles (4 bits)
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x0, _,    _,    _) => self.sys(nnn),
            (0x1, _, _, _)       => self.jp(nnn),
            (0x2, _, _, _)       => self.call(nnn),
            (0x3, _, _, _)       => self.se_kk(x, kk),
            (0x4, _, _, _)       => self.sne_kk(x, kk),
            (0x5, _, _, 0x0)     => self.se_y(x, y),
            (0x6, _, _, _)       => self.ld_ll(x, kk),
            (0x7, _, _, _)       => self.add_kk(x, kk),
            (0x8, _, _, 0x0)     => self.ld_y(x, y),
            (0x8, _, _, 0x1)     => self.or(x, y),
            (0x8, _, _, 0x2)     => self.and(x, y),
            (0x8, _, _, 0x3)     => self.xor(x, y),
            (0x8, _, _, 0x4)     => self.add_y(x, y),
            (0x8, _, _, 0x5)     => self.sub(x, y),
            (0x8, _, _, 0x6)     => self.shr(x, y),
            (0x8, _, _, 0x7)     => self.subn(x, y),
            (0x8, _, _, 0xE)     => self.shl(x, y),
            (0x9, _, _, 0x0)     => self.sne_y(x, y),
            (0xA, _, _, _)       => self.ld_i(nnn),
            (0xB, _, _, _)       => self.jp_v0(nnn),
            (0xC, _, _, _)       => self.rnd(x, kk),
            (0xD, _, _, _)       => self.drw(x, y, n),
            (0xE, _, 0x9, 0xE)   => self.skp(x),
            (0xE, _, 0xA, 0x1)   => self.sknp(x),
            (0xF, _, 0x0, 0x7)   => self.ld_dt(x),
            (0xF, _, 0x0, 0xA)   => self.ld_x(x),
            (0xF, _, 0x1, 0x5)   => self.ld_dt(x),
            (0xF, _, 0x1, 0x8)   => self.ld_st(x),
            (0xF, _, 0x1, 0xE)   => self.add_i(x),
            (0xF, _, 0x2, 0x9)   => self.ld_f(x),
            (0xF, _, 0x3, 0x3)   => self.ld_b(x),
            (0xF, _, 0x5, 0x5)   => self.ld_is(x),
            (0xF, _, 0x6, 0x5)   => self.ld_vx(x),
            _ => return Err(format!("Unknown opcode: {:04X}", opcode)),
        }

        return Ok(());
    }

    fn sys(&mut self, _addr: u16) { 
        // Do nothing
    }

    fn cls(&mut self) {
        self.display.fill(0);
        self.pc += WORD;
    }

    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += WORD; // TODO
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
        self.pc += WORD; // TODO
    }

    fn se_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += WORD;
        }
        self.pc += WORD;
    }
    fn se_y(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += WORD;
        }
        self.pc += WORD;
    }
    
    fn sne_kk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += WORD;
        }
        self.pc += WORD;
    }
    fn sne_y(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += WORD;
        }
        self.pc += WORD;
    }

    fn ld_ll(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc += WORD;
    }
    fn ld_y(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += WORD;
    }
    fn ld_i(&mut self, addr: u16) {
        self.i = addr;
        self.pc += WORD;
    }
    fn ld_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
        self.pc += WORD;
    }
    fn ld_st(&mut self, x: usize) {
        self.v[x] = self.sound_timer;
        self.pc += WORD;
    }
    fn ld_x(&mut self, x: usize) {
        for key in 0..16 {
            if self.keys[key] {
                self.v[x] = key as u8;
                return;
            }
        }

        self.pc -= WORD;
    }
    fn ld_f(&mut self, x: usize) {
        self.i = (self.v[x] as u16) * 5;
        self.pc += WORD;
    }
    fn ld_b(&mut self, x: usize) {
        let v = self.v[x];
        self.memory[self.i as usize] = v / 100;
        self.memory[self.i as usize + 1] = (v / 10) % 10;
        self.memory[self.i as usize + 2] = v % 10;
        self.pc += WORD;
    }
    fn ld_is(&mut self, x: usize) {
        for n in 0..=x {
            self.memory[self.i as usize + n] = self.v[n];
        }
        self.pc += WORD;
    }
    fn ld_vx(&mut self, x: usize) {
        for n in 0..=x {
            self.v[n] = self.memory[self.i as usize + n];
        }
        self.pc += WORD;
    }

    fn add_kk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
        self.pc += WORD;
    }
    fn add_y(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xF] = (overflow) as u8;
        self.v[x] = res;
        self.pc += WORD;
    }
    fn add_i(&mut self, x: usize) {
        self.i = self.i.wrapping_add(self.v[x] as u16);
        self.pc += WORD;
    }

    fn sub(&mut self, x: usize, y: usize) {
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += WORD;
    }
    fn subn(&mut self, x: usize, y: usize) {
        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += WORD;
    }

    fn or(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
        self.pc += WORD;
    }

    fn and(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
        self.pc += WORD;
    }

    fn xor(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
        self.pc += WORD;
    }

    fn shr(&mut self, x: usize, _y: usize) {
        let vx = self.v[x];
        self.v[0xF] = vx & 1;
        self.v[x] = vx >> 1;
        self.pc += WORD;
    }

    fn shl(&mut self, x: usize, _y: usize) {
        let vx = self.v[x];
        self.v[0xF] = (vx >> 7) & 1;
        self.v[x] = vx << 1;
        self.pc += WORD;
    }

    fn rnd(&mut self, x: usize, kk: u8) {
        self.v[x] = rand::random::<u8>() & kk;
        self.pc += WORD;
    }

    fn drw(&mut self, vx: usize, vy: usize, n: u8) {
        self.v[0xF] = 0;

        let start_x = (self.v[vx] as usize) % DISPLAY_HEIGHT;
        let start_y = (self.v[vy] as usize) % DISPLAY_WIDTH;

        for row in 0..n as usize {
            if start_y + row >= DISPLAY_WIDTH { break; }

            let sprite_byte = self.memory[self.i as usize + row];

            for col in 0..8 {
                if start_x + col >= DISPLAY_HEIGHT { break; }

                let pixel_x = start_x + col;
                let pixel_y = start_y + row;

                let byte_idx = pixel_y * 8 + (pixel_x / 8);
                let bit_idx = 7 - (pixel_x % 8);

                let sprite_bit = (sprite_byte >> (7 - col)) & 1;
                let screen_bit = (self.display[byte_idx] >> bit_idx) & 1;

                if screen_bit == 1 && sprite_bit == 1 {
                    self.v[0xF] = 1;
                }

                let new_bit = screen_bit ^ sprite_bit;
                if new_bit == 1 {
                    self.display[byte_idx] |= 1 << bit_idx;
                } else {
                    self.display[byte_idx] &= !(1 << bit_idx);
                }
            }
        }

        self.pc += WORD;
    }

    fn skp(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] == true {
            self.pc += WORD;
        }
        self.pc += WORD;
    }

    fn sknp(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] == false {
            self.pc += WORD;
        }
        self.pc += WORD;
    }
}
