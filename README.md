# Rusty Chip8

This is a straight forward Chip-8 emulator written in Rust using SDL2 for the graphics and platform bindings.

To build and run the emulator issue the following command:

```bash
cargo run --release
```
Note that you will need to source ROM files from the internet and you must acquire the appropriate SDL2 DLL for your system and place it beside the emulator binary -- see  [rust-sdl2 on Github](https://github.com/Rust-SDL2/rust-sdl2) for more details.

## Keypad Layout

The CHIP-8 has a 16-key keypad which I have mapped to the left-most end of the keyboard. This table shows the keymapping in the form CHIP-8 Key (Keyboard key), e.g. pressing W registers as key 0x5 on the emulator.

1 (1) | 2 (2) | 3 (3) | C (4)

4 (Q) | 5 (W) | 6 (E) | D (R)

7 (A) | 8 (S) | 9 (D) | E (F)

A (Z) | 0 (X) | B (C) | F (V)

## Project Organization
This project is split into the following files:

* main.rs - This is the entry point for the application; it instantiates the SDL2 components and the emulator module, gets the ROM from the user, and executes the clock loop.
* chip8.rs - This is the CHIP-8 emulator itself, it implements the CHIP-8 system and opcodes with public interfaces for a frontend to access.
* squarewave.rs - This file contains a square wave struct for use with the SDL2 audio subsystem to produce a single-tone beep.

![test screen](docs/screenshots/chip-8-test.PNG)

![pong](docs/screenshots/pong.gif)