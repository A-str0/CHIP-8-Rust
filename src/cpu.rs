use std::fs;

pub const DISPLAY_HEIGHT: usize = 64;
pub const DISPLAY_WIDTH: usize  = 32;

const MAX_MEMORY: usize = 4096;
const WORD: u16 = 2;

pub struct Chip8CPU {
    memory: [u8; MAX_MEMORY],       // chip RAM
    stack: [u16; 16],               // stack
    display: [u8; (DISPLAY_HEIGHT * DISPLAY_WIDTH) / 8],     // display
    v: [u8; 16],                    // v registries
    i: u16,                         // i registry
    pc: u16,                        // program counter
    sp: u8,                         // stack pointer
    delay_timer: u8,
    sound_timer: u8,
    pub keys: u16,                  // all keys
}

enum PcOpertion {
    NEXT,
    SKIP,
    JUMP(u16)
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

    pub fn new() -> Result<Self, String> {
        let mut cpu = Chip8CPU { 
            memory: [0; MAX_MEMORY], 
            stack: [0; 16],
            display: [0; (DISPLAY_HEIGHT * DISPLAY_WIDTH) / 8],
            v: [0; 16], 
            i: 0, pc: 0x200, sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: 0,
        };
        cpu.load_fonts()?;
        
        Ok(cpu)
    }
    pub fn reset(&mut self) -> Result<(), String> {
        self.memory.fill(0);
        self.stack.fill(0);
        self.display.fill(0);
        self.v.fill(0);
        self.i = 0;
        self.pc = 0x200;
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = 0;

        self.load_fonts()?;
        Ok(())
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), String> {
        let rom = fs::read(path).map_err(|e| format!("Failed to read ROM: {}", e))?;

        if rom.len() > MAX_MEMORY - 0x200 {
            return Err(format!("ROM is too big! ({})", rom.len()));
        }
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);

        Ok(())
    }
    fn load_fonts(&mut self) -> Result<(), String> {
        let fonts = fs::read("chip8_fonts.bin").map_err(|e| format!("Failed to read Fonts: {}", e))?;

        if fonts.len() > 80 {
            return Err(format!("Fonts binary file is corrupted: {}", fonts.len()));
        }
        self.memory[0..80].copy_from_slice(&fonts);

        Ok(())
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch_oppcode();

        self.execute(opcode).expect("error lol");
    }

    fn fetch_oppcode(&mut self) -> u16 {
        if usize::from(self.pc) >= MAX_MEMORY - 1 { return 0; }

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

        let pc_op = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x0, _,    _,    _) => self.sys(nnn),
            (0x1, _, _, _)       => self.jp(nnn),
            (0x2, _, _, _)       => self.call(nnn),
            (0x3, _, _, _)       => self.se_kk(x, kk),
            (0x4, _, _, _)       => self.sne_kk(x, kk),
            (0x5, _, _, 0x0)     => self.se_y(x, y),
            (0x6, _, _, _)       => self.ld_kk(x, kk),
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
        };

        match pc_op {
            PcOpertion::NEXT            => self.pc += WORD,
            PcOpertion::SKIP            => self.pc += 2 * WORD,
            PcOpertion::JUMP(addr) => self.pc = addr,
        };

        Ok(())
    }

    fn sys(&mut self, _addr: u16) -> PcOpertion { 
        // Do nothing
        PcOpertion::NEXT
    }

    fn cls(&mut self) -> PcOpertion {
        self.display.fill(0);
        PcOpertion::NEXT
    }

    fn ret(&mut self) -> PcOpertion {
        self.sp -= 1;
        PcOpertion::JUMP(self.stack[self.sp as usize])
    }

    fn jp(&mut self, addr: u16) -> PcOpertion {
        PcOpertion::JUMP(addr)
    }
    fn jp_v0(&mut self, addr: u16) -> PcOpertion { 
        PcOpertion::JUMP(addr + self.v[0x0] as u16)
    }

    fn call(&mut self, addr: u16) -> PcOpertion {
        self.stack[self.sp as usize] = self.pc + WORD;
        self.sp += 1;
        PcOpertion::JUMP(addr)
    }

    fn se_kk(&mut self, x: usize, kk: u8) -> PcOpertion {
        if self.v[x] == kk {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }
    fn se_y(&mut self, x: usize, y: usize) -> PcOpertion {
        if self.v[x] == self.v[y] {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }
    
    fn sne_kk(&mut self, x: usize, kk: u8) -> PcOpertion {
        if self.v[x] != kk {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }
    fn sne_y(&mut self, x: usize, y: usize) -> PcOpertion {
        if self.v[x] != self.v[y] {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }

    fn ld_kk(&mut self, x: usize, kk: u8) -> PcOpertion {
        self.v[x] = kk;
        PcOpertion::NEXT
    }
    fn ld_y(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[x] = self.v[y];
        PcOpertion::NEXT
    }
    fn ld_i(&mut self, addr: u16) -> PcOpertion {
        self.i = addr;
        PcOpertion::NEXT
    }
    fn ld_dt(&mut self, x: usize) -> PcOpertion {
        self.v[x] = self.delay_timer;
        PcOpertion::NEXT
    }
    fn ld_st(&mut self, x: usize) -> PcOpertion {
        self.v[x] = self.sound_timer;
        PcOpertion::NEXT
    }
    fn ld_x(&mut self, x: usize) -> PcOpertion {
        for key in 0..16 {
            if (self.keys & (1 << key)) != 0 {
                self.v[x] = key as u8;
                return PcOpertion::NEXT;
            }
        }

        PcOpertion::JUMP(self.pc)
    }
    fn ld_f(&mut self, x: usize) -> PcOpertion {
        self.i = (self.v[x] as u16) * 5;
        PcOpertion::NEXT
    }
    fn ld_b(&mut self, x: usize) -> PcOpertion {
        let v = self.v[x];
        self.memory[self.i as usize] = v / 100;
        self.memory[self.i as usize + 1] = (v / 10) % 10;
        self.memory[self.i as usize + 2] = v % 10;
        PcOpertion::NEXT
    }
    fn ld_is(&mut self, x: usize) -> PcOpertion {
        for n in 0..=x {
            self.memory[self.i as usize + n] = self.v[n];
        }
        PcOpertion::NEXT
    }
    fn ld_vx(&mut self, x: usize) -> PcOpertion {
        for n in 0..=x {
            self.v[n] = self.memory[self.i as usize + n];
        }
        PcOpertion::NEXT
    }

    fn add_kk(&mut self, x: usize, kk: u8) -> PcOpertion {
        self.v[x] = self.v[x].wrapping_add(kk);
        PcOpertion::NEXT
    }
    fn add_y(&mut self, x: usize, y: usize) -> PcOpertion {
        let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xF] = if overflow {1} else {0};
        self.v[x] = res;
        PcOpertion::NEXT
    }
    fn add_i(&mut self, x: usize) -> PcOpertion {
        self.i = self.i.wrapping_add(self.v[x] as u16);
        self.v[0xF] = if self.i > 0x0F00 { 1 } else { 0 };
        PcOpertion::NEXT
    }

    fn sub(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        PcOpertion::NEXT
    }
    fn subn(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        PcOpertion::NEXT
    }

    fn or(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[x] |= self.v[y];
        PcOpertion::NEXT
    }

    fn and(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[x] &= self.v[y];
        PcOpertion::NEXT
    }

    fn xor(&mut self, x: usize, y: usize) -> PcOpertion {
        self.v[x] ^= self.v[y];
        PcOpertion::NEXT
    }

    fn shr(&mut self, x: usize, _y: usize) -> PcOpertion {
        self.v[0xF] = self.v[x] & 1;
        self.v[x] >>= 1;
        PcOpertion::NEXT
    }

    fn shl(&mut self, x: usize, _y: usize) -> PcOpertion {
        self.v[0xF] = (self.v[x] >> 7) & 1;
        self.v[x] <<= 1;
        PcOpertion::NEXT
    }

    fn rnd(&mut self, x: usize, kk: u8) -> PcOpertion {
        self.v[x] = rand::random::<u8>() & kk;
        PcOpertion::NEXT
    }

    fn drw(&mut self, vx: usize, vy: usize, n: u8) -> PcOpertion {
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

        PcOpertion::NEXT
    }

    fn skp(&mut self, x: usize) -> PcOpertion {
        if (self.keys & (1 << self.v[x])) != 0 {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }

    fn sknp(&mut self, x: usize) -> PcOpertion {
        if (self.keys & (1 << self.v[x])) == 0 {
            return PcOpertion::SKIP;
        }
        PcOpertion::NEXT
    }
}
