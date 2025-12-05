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
pub const DISPLAY_SIZE_Y: usize = 64;
pub const DISPLAY_SIZE_X: usize = 32;
const WORD: u16 = 2;
pub struct Chip8CPU {
    memory: [u8; 4096],     // chip RAM
    stack: [u16; 16],       // stack
    display: [u8; DISPLAY_SIZE_Y * DISPLAY_SIZE_Y / 8],     // display
    v: [u8; 16],            // v registries
    i: u16,                 // i registry
    pc: u16,                // program counter
    sp: u8,                 // stack pointer

    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; 16],       // all keys
}

impl Chip8CPU {
    pub fn new() -> Self {
        let mut cpu: Chip8CPU = Chip8CPU { 
            memory: [0; 4096], 
            stack: [0; 16],
            display: [0; DISPLAY_SIZE_Y * DISPLAY_SIZE_Y / 8],
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
        let oppcode = self.fetch_oppcode();

        self.execute(oppcode).expect("error lol");

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        } 
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        } 

    }

    fn fetch_oppcode(&mut self) -> u16 {
        if self.pc >= 4096 { return 0; }

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
        let n      = (opcode & 0x000F) as u8;

        // oppcode is 16 bytes so it can be separeted into 4 nibbles (полубайт, если по-русски)
        // in our case nibble is type with four nibbles (4 bits)
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );

        match  nibbles {
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
    fn ld_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }
    fn ld_st(&mut self, x: usize) {
        self.v[x] = self.sound_timer;
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
    }
    fn ld_b(&mut self, x: usize) {
        let v = self.v[x];
        self.memory[self.i as usize] = v / 100;
        self.memory[self.i as usize + 1] = (v / 10) % 10;
        self.memory[self.i as usize + 2] = v % 10;
    }
    fn ld_is(&mut self, x: usize) {
        for n in 0..=x {
            self.memory[self.i as usize + n] = self.v[n];
        }
    }
    fn ld_vx(&mut self, x: usize) {
        for n in 0..=x {
            self.v[n] = self.memory[self.i as usize + n];
        }
    }


    fn add_kk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }
    fn add_y(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xF] = (overflow) as u8;
        self.v[x] = res;
    }
    fn add_i(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }

    fn sub(&mut self, x: usize, y: usize) {
        self.v[0xF] = (self.v[x] > self.v[y]) as u8;
        self.v[x] -=  self.v[y];
    }
    fn subn(&mut self, x: usize, y: usize) {
        self.v[0xF] = (self.v[y] > self.v[x]) as u8;
        self.v[x] =  self.v[y] - self.v[x];
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

    fn shr(&mut self, x: usize, _y: usize) {
        let vx = self.v[x];
        self.v[0xF] = vx & 1;
        self.v[x] = vx >> 1;
    }

    fn shl(&mut self, x: usize, _y: usize) {
        let vx = self.v[x];
        self.v[0xF] = (vx >> 7) & 1;
        self.v[x] = vx << 1;
    }

    fn rnd(&mut self, x: usize, kk: u8) {
        self.v[x] = rand::random::<u8>() & kk;
    }

    fn drw(&mut self, x: usize, y: usize, n: u8) {
        let sprite_addr = self.i as usize;
        let pos_x = (self.v[x] as usize) % DISPLAY_SIZE_Y;
        let pos_y = (self.v[y] as usize) % DISPLAY_SIZE_Y;

        self.v[0xF] = 0;

        for row in 0..n as usize {
            let sprite_byte = self.memory[sprite_addr + row];

            for bit in 0..8 {
                let pixel_x = (pos_x + bit) % DISPLAY_SIZE_Y;
                let pixel_y = (pos_y + row) % DISPLAY_SIZE_Y;

                let idx = pixel_y * DISPLAY_SIZE_Y + pixel_x;

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

    fn skp(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] == true {
            self.pc += WORD * 2;
        }
    }

    fn sknp(&mut self, x: usize) {
        if self.keys[self.v[x] as usize] == false {
            self.pc += WORD * 2;
        }
    }
}
