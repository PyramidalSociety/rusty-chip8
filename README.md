<!-- markdownlint-disable MD013 -->

# Rusty CHIP-8

A terminal-based (TUI) CHIP-8 emulator written in Rust using the [Ratatui](https://ratatui.rs) library

CHIP-8 is an interpreted programming language developed by Joseph Weisbecker in the mid 70s. The language uses hexadecimal codes for instructions, making it look similar to machine code. CHIP-8 interpreters have been developed for many devices such as computers, microcomputers, graphing calculators, mobile phones and video game consoles.

## Why this project

The main motivation behind the project was to learn lower level programming concepts and to get more familiar with the Rust programming language.

Here are some concepts I learned while writing this program:

- How a CPU implements fetch, decode and execute
- How a CPU can utilize memory, stack, program counters, stack pointers, memory addresses, and registers
- How to disassemble and decode an opcode into instructions a CPU can use
- Becoming familiar with external Rust libraries, Rust's module system, error handling and test-driven development

## Screenshots

![Rush Hour](images/RushHour.png "Rush Hour")
_Rush Hour_

![Slippery Slope](images/SlipperySlope.png "Slippery Slope")
_Slippery Slope_

![Space Invaders](images/SpaceInvaders.png "Space Invaders")
_Space Invaders_

## Installation

You can run the emulator by using the precompiled binaries from the [releases page](https://git.py-cloud.net/Pyramidal/rusty-chip8/releases) or compile and install it from source using `cargo`

### Installation using `cargo`

First, install Rust and Cargo using the [rustup installer](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Cloning and installing

```bash
git clone https://git.py-cloud.net/Pyramidal/rusty-chip8.git
cd rusty-chip8
cargo install --path .
```

## Usage

```bash
rusty-chip8 <path-to-ROM>
```

## Keybindings

The CHIP-8 uses a 16-key hexadecimal keypad, mapped to the keyboard as follows:

|  CHIP-8 Keypad  |    Keyboard     |
| :-------------: | :-------------: |
| `1` `2` `3` `C` | `1` `2` `3` `4` |
| `4` `5` `6` `D` | `Q` `W` `E` `R` |
| `7` `8` `9` `E` | `A` `S` `D` `F` |
| `A` `0` `B` `F` | `Z` `X` `C` `V` |
