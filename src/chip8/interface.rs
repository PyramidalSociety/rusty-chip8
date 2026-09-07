use std::time::Duration;

use super::*;

impl Chip8 {
    pub fn get_video(&self) -> &[bool; VIDEO_SIZE] {
        &self.video
    }

    pub fn decrease_delay_timer(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
            self.last_delay_decrease = Some(Instant::now());
        }

        if self.delay_timer == 0 {
            self.last_delay_decrease = None;
        }
    }

    pub fn decrease_sound_timer(&mut self) {
        if self.sound_timer != 0 {
            self.sound_timer -= 1;
            self.last_sound_decrease = Some(Instant::now());
        }

        if self.sound_timer == 0 {
            self.last_sound_decrease = None;
        }
    }

    pub fn sound_on(&self) -> bool {
        self.sound_timer != 0
    }

    pub fn get_elapsed_sound(&self) -> Duration {
        match self.last_sound_decrease {
            Some(last_timestamp) => last_timestamp.elapsed(),
            None => Duration::from_nanos(0),
        }
    }

    pub fn get_epalsed_delay(&self) -> Duration {
        match self.last_delay_decrease {
            Some(last_timestamp) => last_timestamp.elapsed(),
            None => Duration::from_nanos(0),
        }
    }
}
