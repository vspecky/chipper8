use crate::WIN_WIDTH;
use crate::WIN_HEIGHT;
use crate::PIXEL_SIZE;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

pub struct GUI {
    canvas: Canvas<Window>,
    events: EventPump,
}

impl GUI {
    pub fn new() -> Self {
        let sdl_ctx = sdl2::init().unwrap();
        let event_pump = sdl_ctx.event_pump().unwrap();
        let video_subsys = sdl_ctx.video().unwrap();

        let window = video_subsys.window("Chipper 8", WIN_WIDTH * PIXEL_SIZE, WIN_HEIGHT * PIXEL_SIZE)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Self {
            canvas: canvas,
            events: event_pump
        }
    }

    pub fn draw(&mut self, screen: &[[bool; WIN_WIDTH as usize]; WIN_HEIGHT as usize]) {
        for y in 0..WIN_HEIGHT {
            let row = screen[y as usize];

            for x in 0..WIN_WIDTH {
                let pixel_val = row[x as usize];

                let col = self.get_color(pixel_val);
                let rect = Rect::new((x * PIXEL_SIZE) as i32 , (y * PIXEL_SIZE) as i32, PIXEL_SIZE, PIXEL_SIZE);
                self.canvas.set_draw_color(col);
                self.canvas.fill_rect(rect).expect("Error drawing to the screen");
            }
        }

        self.canvas.present();
    }

    pub fn get_keypad_state(&mut self) -> Option<[bool; 16]> {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} => return None,
                _ => ()
            }
        }

        let mut keypad = [false; 16];

        let keys: Vec<Keycode> = self.events
                                    .keyboard_state()
                                    .pressed_scancodes()
                                    .filter_map(Keycode::from_scancode)
                                    .collect();

        for key in keys {
            match key {
                Keycode::Num1 => keypad[0x1] = true,
                Keycode::Num2 => keypad[0x2] = true,
                Keycode::Num3 => keypad[0x3] = true,
                Keycode::Num4 => keypad[0xC] = true,
                Keycode::Q => keypad[0x4] = true,
                Keycode::W => keypad[0x5] = true,
                Keycode::E => keypad[0x6] = true,
                Keycode::R => keypad[0xD] = true,
                Keycode::A => keypad[0x7] = true,
                Keycode::S => keypad[0x8] = true,
                Keycode::D => keypad[0x9] = true,
                Keycode::F => keypad[0xE] = true,
                Keycode::Z => keypad[0xA] = true,
                Keycode::X => keypad[0x0] = true,
                Keycode::C => keypad[0xB] = true,
                Keycode::V => keypad[0xF] = true,
                _ => ()
            }
        }

        Some(keypad)
    }

    fn get_color(&self, pixel: bool) -> Color {
        if pixel {
            Color::RGB(255, 255, 255)
        } else {
            Color::RGB(0, 0, 0)
        }
    }
}
