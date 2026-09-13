use crate::chip8::Chip8;
use std::{
    io::{self, stdout},
    path::Path,
};

mod beeper;
mod draw;
mod input;
pub mod run;

use beeper::Beeper;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement},
};

pub struct App {
    chip8: Chip8,
    beeper: Beeper,
    enhanced: bool,
    exit: bool,
}

fn install_panic_hook() {
    let original = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut out = stdout();
        let _ = execute!(out, PopKeyboardEnhancementFlags);

        let _ = disable_raw_mode();

        original(info);
    }));
}

impl App {
    pub fn new(pth: &Path) -> io::Result<App> {
        install_panic_hook();

        enable_raw_mode()?;

        let mut out = stdout();

        let obj = Self {
            chip8: Chip8::new(&pth)?,
            beeper: Beeper::new(),
            enhanced: supports_keyboard_enhancement().unwrap_or_else(|_| false),
            exit: false,
        };

        if obj.enhanced {
            execute!(
                out,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                )
            )?;
        }

        Ok(obj)
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let mut out = stdout();

        if self.enhanced {
            let _ = execute!(out, PopKeyboardEnhancementFlags);
        }

        let _ = disable_raw_mode();
    }
}
