mod chip8;
mod gui;
//mod disassembler;
//use disassembler::Disassembler;
use std::vec::Vec;
use std::fs::{self, File};
use std::io::prelude::*;
use std::time::Duration;

const WIN_WIDTH: u32 = 64;
const WIN_HEIGHT: u32 = 32;
const PIXEL_SIZE: u32 = 15;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let program_bytes = get_program_bytes(args).expect("Problem reading file");

    // let mut disass = Disassembler::new(&program_bytes);

    // println!("{}", disass.disassemble());
    
    let mut cpu = chip8::CHIP8::new(program_bytes);
    
    let mut win = gui::GUI::new();

    let delay = Duration::from_millis(2);

    loop {
        match win.get_keypad_state() {
            Some(keypad) => {
                let redraw = cpu.tick(keypad);

                if redraw {
                    win.draw(&cpu.screen);
                }
            }

            None => break,
        }

        std::thread::sleep(delay);
    }
}

fn get_program_bytes(args: Vec<String>) -> Result<Vec<u8>, String> {
    if args.len() != 2 {
        return Err(String::from("Invalid argument length"));
    }

    let mut file = File::open(&args[1]).expect("File not found.");
    let meta = fs::metadata(&args[1]).expect("Can't read file metadata");

    let mut bytes = vec![0; meta.len() as usize];

    match file.read(&mut bytes) {
        Err(msg) => return Err(msg.to_string()),
        _ => ()
    }

    //println!("{:x?}", &bytes[..]);

    Ok(bytes)
}