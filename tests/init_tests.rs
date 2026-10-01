use rusty_chip8::app::App;
use std::path;

#[test]
fn valid_init() {
    assert!(App::new(path::Path::new("tests/fixtures/test_opcode.ch8")).is_ok());
}

#[test]
fn long_init() {
    assert!(App::new(path::Path::new("tests/fixtures/too_big.ch8")).is_err());
}

#[test]
fn inexistent_file_init() {
    assert!(App::new(path::Path::new("tests/fixtures/inexistent.ch8")).is_err());
}
