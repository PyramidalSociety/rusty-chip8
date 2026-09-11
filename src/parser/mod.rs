use std::path::PathBuf;

use clap::{Arg, command, value_parser};

pub fn get_path() -> PathBuf {
    let mut matches = command!()
        .about("A CHIP-8 emulator TUI written in Rust")
        .arg(
            Arg::new("path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();

    matches
        .remove_one::<PathBuf>("path")
        .expect("Argument is required")
}
