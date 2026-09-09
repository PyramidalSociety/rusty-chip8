use crate::chip8::Chip8;
use std::{io, path::Path};

mod beeper;
mod draw;
mod input;
pub mod run;

use beeper::Beeper;

pub struct App {
    chip8: Chip8,
    beeper: Beeper,
    exit: bool,
}

impl App {
    pub fn new(pth: &Path) -> io::Result<App> {
        Ok(Self {
            chip8: Chip8::new(&pth)?,
            beeper: Beeper::new(),
            exit: false,
        })
    }
}
