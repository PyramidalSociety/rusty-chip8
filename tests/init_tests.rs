use rusty_chip8::chip8::Chip8;
use std::path;

#[test]
fn valid_init() {
    assert!(Chip8::new(path::Path::new("tests/fixtures/test_opcode.ch8")).is_ok());
}

#[test]
fn long_init() {
    assert!(Chip8::new(path::Path::new("tests/fixtures/too_big.ch8")).is_err());
}

#[test]
fn inexistent_file_init() {
    assert!(Chip8::new(path::Path::new("tests/fixtures/inexistent.ch8")).is_err());
}
