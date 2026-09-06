use std::time::Duration;

use super::*;

const KEY_TIMEOUT: Duration = Duration::from_millis(150);

impl Chip8 {
    pub fn press_key(&mut self, key: u8) {
        if key as usize >= KEYPAD_SIZE {
            return;
        }

        self.keypad[key as usize] = true;
        self.last_keypress[key as usize] = Some(Instant::now());
    }

    pub(super) fn unpress_key(&mut self) {
        for key in 0..KEYPAD_SIZE {
            if let Some(last) = self.last_keypress[key as usize] {
                if last.elapsed() > KEY_TIMEOUT {
                    self.last_keypress[key as usize] = None;
                    self.keypad[key as usize] = false;
                }
            }
        }
    }
}
