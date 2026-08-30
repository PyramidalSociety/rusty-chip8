use std::{fs, io, path};

const START_ADDRESS: usize = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;

const NUM_REGISTERS: usize = 16;
const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;
const VIDEO_SIZE: usize = 64 * 32;

pub struct Chip8 {
    registers: [u8; NUM_REGISTERS],
    memory: [u8; MEM_SIZE],
    index: u16,
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; KEYPAD_SIZE],
    video: [bool; VIDEO_SIZE],
}

// Creating a new Chip8 instance
impl Chip8 {
    pub fn new(pth: &path::Path) -> Result<Self, io::Error> {
        let mut new_obj = Chip8 {
            registers: [0; NUM_REGISTERS],
            memory: [0; MEM_SIZE],
            index: 0,
            pc: START_ADDRESS as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; KEYPAD_SIZE],
            video: [false; VIDEO_SIZE],
        };

        new_obj.load_rom(pth)?;

        Ok(new_obj)
    }

    fn load_rom(&mut self, pth: &path::Path) -> io::Result<()> {
        let bytes = fs::read(pth)?;

        let end = START_ADDRESS + bytes.len();

        if end > self.memory.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "ROM size exceeds available Chip-8 RAM space",
            ));
        }

        self.memory[START_ADDRESS..end].copy_from_slice(&bytes);

        let fontset: [u8; FONTSET_SIZE] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let end = FONTSET_START_ADDRESS + FONTSET_SIZE;

        self.memory[FONTSET_START_ADDRESS..end].copy_from_slice(&fontset);

        Ok(())
    }
}

impl Chip8 {
    fn cls_00e0(&mut self) {
        self.video[..].fill(false);
    }

    fn ret_00ee(&mut self) {
        if self.sp == 0 {
            panic!("Stack pointer underflow");
        }

        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn jp_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn call_2nnn(&mut self, nnn: u16) {
        if self.sp as usize == STACK_SIZE {
            panic!("Stack overflow");
        }

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn se_3xkk(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.pc += 2;
        }
    }

    fn sne_4xkk(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.pc += 2;
        }
    }

    fn se_5xy0(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.pc += 2;
        }
    }

    fn ld_6xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    fn add_7xkk(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = self.registers[x as usize].overflowing_add(kk).0;
    }

    fn ld_8xy0(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn or_8xy1(&mut self, x: u8, y: u8) {
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn and_8xy2(&mut self, x: u8, y: u8) {
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn xor_8xy3(&mut self, x: u8, y: u8) {
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn add_8xy4(&mut self, x: u8, y: u8) {
        let sum = self.registers[x as usize].overflowing_add(self.registers[y as usize]);

        self.registers[0xF] = sum.1 as u8;
        self.registers[x as usize] = sum.0;
    }

    fn sub_8xy5(&mut self, x: u8, y: u8) {
        let diff = self.registers[x as usize].overflowing_sub(self.registers[y as usize]);

        self.registers[0xF] = diff.1 as u8;
        self.registers[x as usize] = diff.0;
    }

    fn shr_8xy6(&mut self, x: u8) {
        self.registers[0xF] = self.registers[x as usize] & 1;

        self.registers[x as usize] >>= 1;
    }

    fn subn_8xy7(&mut self, x: u8, y: u8) {
        let diff = self.registers[y as usize].overflowing_sub(self.registers[x as usize]);

        self.registers[0xF] = diff.1 as u8;
        self.registers[x as usize] = diff.0;
    }

    fn shl_8xye(&mut self, x: u8) {
        self.registers[0xF] = (self.registers[x as usize] & 0x80) >> 0x7;

        self.registers[x as usize] <<= 1;
    }

    fn ld_i_annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    fn jp_v0_bnnn(&mut self, nnn: u16) {
        self.pc = self.registers[0] as u16 + nnn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_mem() {
        let dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        assert_eq!(dummy.memory[FONTSET_START_ADDRESS], 0xF0);
        assert_eq!(dummy.memory[FONTSET_START_ADDRESS + FONTSET_SIZE - 1], 0x80);
        assert_eq!(dummy.memory[START_ADDRESS], 0x12);
    }

    #[test]
    fn cls_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        for i in 0..2048 {
            dummy.video[i] = true;
        }
        dummy.cls_00e0();

        for i in 0..2048 {
            assert_eq!(dummy.video[i], false);
        }
    }

    #[test]
    fn assignation_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(2, 254);
        assert_eq!(dummy.registers[2], 254);

        dummy.add_7xkk(2, 1);
        assert_eq!(dummy.registers[2], 255);

        dummy.add_7xkk(2, 10);
        assert_eq!(dummy.registers[2], 9);

        assert_eq!(dummy.registers[0], 0);
    }

    #[test]
    fn instr_8xyd() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 11);
        dummy.ld_8xy0(0, 1);
        assert_eq!(dummy.registers[0], dummy.registers[1]);
        assert_eq!(dummy.registers[0], 11);

        dummy.ld_6xkk(0, 8);
        dummy.ld_6xkk(1, 7);
        dummy.or_8xy1(0, 1);
        assert_eq!(dummy.registers[0], 15);

        dummy.ld_6xkk(0, 14);
        dummy.ld_6xkk(1, 7);
        dummy.and_8xy2(0, 1);
        assert_eq!(dummy.registers[0], 6);

        dummy.ld_6xkk(0, 14);
        dummy.ld_6xkk(1, 7);
        dummy.xor_8xy3(0, 1);
        assert_eq!(dummy.registers[0], 9);

        dummy.ld_6xkk(0, 255);
        dummy.ld_6xkk(1, 10);
        dummy.add_8xy4(0, 1);
        assert_eq!(dummy.registers[0], 9);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 32);
        dummy.ld_6xkk(1, 64);
        dummy.add_8xy4(0, 1);
        assert_eq!(dummy.registers[0], 96);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 9);
        dummy.sub_8xy5(0, 1);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 9);
        dummy.ld_6xkk(1, 10);
        dummy.sub_8xy5(0, 1);
        assert_eq!(dummy.registers[0], 255);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 7);
        dummy.shr_8xy6(0);
        assert_eq!(dummy.registers[0], 3);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 6);
        dummy.shr_8xy6(0);
        assert_eq!(dummy.registers[0], 3);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 9);
        dummy.ld_6xkk(1, 10);
        dummy.subn_8xy7(0, 1);
        assert_eq!(dummy.registers[0], 1);
        assert_eq!(dummy.registers[0xF], 0);

        dummy.ld_6xkk(0, 10);
        dummy.ld_6xkk(1, 9);
        dummy.subn_8xy7(0, 1);
        assert_eq!(dummy.registers[0], 255);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 255);
        dummy.shl_8xye(0);
        assert_eq!(dummy.registers[0], 254);
        assert_eq!(dummy.registers[0xF], 1);

        dummy.ld_6xkk(0, 127);
        dummy.shl_8xye(0);
        assert_eq!(dummy.registers[0], 254);
        assert_eq!(dummy.registers[0xF], 0);
    }

    #[test]
    fn ld_i_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_i_annn(1024);
        assert_eq!(dummy.index, 1024);
    }

    #[test]
    fn jp_v0_test() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.ld_6xkk(0, 0xFF);
        dummy.jp_v0_bnnn(0x400);
        assert_eq!(dummy.pc, 0x4FF);
    }
}
