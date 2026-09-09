use std::io;

use rusty_chip8::{app::App, parser};

fn run_app() -> io::Result<()> {
    let pth = parser::get_path();
    let terminal = ratatui::init();
    App::new(pth.as_path())?.run(terminal)
}

fn main() -> io::Result<()> {
    let app_result = run_app();
    ratatui::restore();
    app_result
}
