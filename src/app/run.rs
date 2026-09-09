use ratatui::DefaultTerminal;

use std::{
    io, thread,
    time::{Duration, Instant},
};

use super::*;

const TIMEOUT_TIMER: Duration = Duration::from_nanos(16666667);
const TIMEOUT_OPERATION: Duration = Duration::from_nanos(1428571);

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        let mut last_op = Instant::now();

        let mut last_area = terminal.size()?;

        while !self.exit {
            self.get_input()?;
            let draw_instr = self.chip8.run();

            if self.chip8.sound_on() {
                self.beeper.play();
            } else {
                self.beeper.pause();
            }

            if last_area != terminal.size()? || draw_instr {
                last_area = terminal.size()?;
                terminal.draw(|frame| self.draw(frame))?;
            }

            if self.chip8.get_epalsed_delay() >= TIMEOUT_TIMER {
                self.chip8.decrease_delay_timer();
            }

            if self.chip8.get_elapsed_sound() >= TIMEOUT_TIMER {
                self.chip8.decrease_sound_timer();
            }

            let elapsed = last_op.elapsed();
            if elapsed < TIMEOUT_OPERATION {
                thread::sleep(TIMEOUT_OPERATION - elapsed);
            }
            last_op = Instant::now();
        }
        Ok(())
    }
}
