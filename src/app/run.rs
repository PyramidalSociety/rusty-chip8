use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use std::{
    io, thread,
    time::{Duration, Instant},
};

use super::*;

const TIMEOUT_TIMER: Duration = Duration::from_nanos(16666667);
const TIMEOUT_OPERATION: Duration = Duration::from_nanos(1428571);

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

    fn get_input(&mut self) -> io::Result<()> {
        self.chip8.unpress_key();

        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    self.exit = true;
                    continue;
                }

                if key.kind == KeyEventKind::Release {
                    continue;
                }

                if let Some(key) = map_key(key.code) {
                    self.chip8.press_key(key);
                }
            }
        }

        Ok(())
    }
}
