use super::*;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use std::{io, time::Duration};

fn map_key(key: KeyCode) -> Option<u8> {
    match key {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('v') => Some(0xF),
        _ => None,
    }
}

impl App {
    pub(super) fn get_input(&mut self) -> io::Result<()> {
        if !self.enhanced {
            self.chip8.unpress_key_timeout();
        }

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    self.exit = true;
                    continue;
                }

                if key.kind == KeyEventKind::Release {
                    if let Some(key) = map_key(key.code) {
                        self.chip8.unpress_key(key);
                    }

                    continue;
                }

                if let Some(key) = map_key(key.code) {
                    self.chip8.press_key(key, self.enhanced);
                }
            }
        }

        Ok(())
    }
}
