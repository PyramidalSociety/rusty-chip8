use crate::chip8::Chip8;
use std::{io, path::Path};

mod draw;

pub struct App {
    chip8: Chip8,
    exit: bool,
}

impl App {
    pub fn new(pth: &Path) -> io::Result<App> {
        Ok(Self {
            chip8: Chip8::new(&pth)?,
            exit: false,
        })
    }
}
