mod chip8;
mod gui;
use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use sdl2::pixels::Color;

const WIN_WIDTH: u32 = 64;
const WIN_HEIGHT: u32 = 32;
const PIXEL_SIZE: u32 = 10;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let program_bytes = get_program_bytes(args).expect("Problem reading file");

    let mut cpu = chip8::CHIP8::new(program_bytes);
    let mut win = gui::GUI::new();

    let delay = Duration::from_millis(1);

    loop {
        match win.get_keypad_state() {
            Some(keypad) => {
                let needta_draw = cpu.tick(keypad);

                if needta_draw {
                    win.draw(&cpu.screen);
                }
            }

            None => break,
        }

        std::thread::sleep(delay);
    }
}

fn get_program_bytes(args: Vec<String>) -> Result<[u8; 3584], ()> {
    if args.len() != 2 {
        return Err(());
    }

    let mut bytes = [0_u8; 3584];

    let mut file = File::open(&args[1]).expect("File not found.");

    match file.read(&mut bytes) {
        Err(_) => return Err(()),
        _ => ()
    }

    Ok(bytes)
}