use std::{fs, io, path};

const START_ADDRESS: usize = 0x200;

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
        Ok(())
    }
}
