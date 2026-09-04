use std::{fs, io, path};

mod cycle;
mod instructions;
pub mod interface;

const START_ADDRESS: usize = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;

const NUM_REGISTERS: usize = 16;
const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const KEYPAD_SIZE: usize = 16;
const VIDEO_SIZE: usize = 64 * 32;
const VIDEO_HEIGHT: usize = 32;
const VIDEO_WIDTH: usize = 64;

pub struct Chip8 {
    registers: [u8; NUM_REGISTERS],
    memory: [u8; MEM_SIZE],
    index: u16,
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; KEYPAD_SIZE],
    video: [bool; VIDEO_SIZE],
}

impl Chip8 {
    pub fn new(pth: &path::Path) -> io::Result<Chip8> {
        let mut new_obj = Chip8 {
            registers: [0; NUM_REGISTERS],
            memory: [0; MEM_SIZE],
            index: 0,
            pc: START_ADDRESS as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; KEYPAD_SIZE],
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
}
