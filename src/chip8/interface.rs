use super::*;

impl Chip8 {
    pub fn get_video(&self) -> &[bool; VIDEO_SIZE] {
        &self.video
    }

    pub fn decrease_delay_timer(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn decrease_sound_timer(&mut self) {
        if self.sound_timer != 0 {
            self.sound_timer -= 1;
        }
    }
}
