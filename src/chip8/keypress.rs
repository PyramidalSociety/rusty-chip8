use std::time::Duration;

use super::*;

const KEY_TIMEOUT: Duration = Duration::from_millis(100);

impl Chip8 {
    pub fn press_key(&mut self, key: u8, enhanced: bool) {
        if key as usize >= KEYPAD_SIZE {
            return;
        }

        self.keypad[key as usize] = true;

        if !enhanced {
            self.last_keypress[key as usize] = Some(Instant::now());
        }
    }

    pub fn unpress_key_timeout(&mut self) {
        for key in 0..KEYPAD_SIZE {
            if let Some(last) = self.last_keypress[key as usize] {
                if last.elapsed() > KEY_TIMEOUT {
                    self.last_keypress[key as usize] = None;
                    self.keypad[key as usize] = false;
                }
            }
        }
    }

    pub fn unpress_key(&mut self, key: u8) {
        if key as usize >= KEYPAD_SIZE {
            return;
        }

        self.keypad[key as usize] = false;
    }
}
