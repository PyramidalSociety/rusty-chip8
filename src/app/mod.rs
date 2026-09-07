use crate::chip8::Chip8;
use std::{error::Error, path::Path};

mod beeper;
mod draw;
pub mod run;

use beeper::Beeper;

pub struct App {
    chip8: Chip8,
    beeper: Beeper,
    exit: bool,
}

impl App {
    pub fn new(pth: &Path) -> Result<App, Box<dyn Error>> {
        Ok(Self {
            chip8: Chip8::new(&pth)?,
            beeper: Beeper::new(),
            exit: false,
        })
    }
}
