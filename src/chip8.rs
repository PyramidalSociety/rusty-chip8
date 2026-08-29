use std::{fs, io, path};

const START_ADDRESS: usize = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    video: [bool; 64 * 32],
    opcode: u16,
}

// Creating a new Chip8 instance
impl Chip8 {
    pub fn new(pth: &path::Path) -> Result<Self, io::Error> {
        let mut new_obj = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [false; 64 * 32],
            opcode: 0x200,
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
    fn clear_display() {
        let mut dummy = Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).unwrap();

        dummy.video[0] = true;
        dummy.cls_00e0();

        assert_eq!(dummy.video[0], false);
    }
}
