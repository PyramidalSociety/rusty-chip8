use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;
use std::io;

use super::*;

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            while event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        self.exit = true;
                    }
                }
            }

            self.chip8.run();
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }
}
