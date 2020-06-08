use rand::Rng;

pub struct CHIP8 {
    mem: [u8; 4096],  // Memory for Chip-8 (4 KB)
    vx: [u8; 16],     // General Purpose Registers (V0 - VF)
    stk: [u16; 16],   // Stack
    pc: u16,          // Program Counter
    sp: i16,          // Stack Pointer
    i: u16,           // Index Register (Used for storing memory addresses)
    dt: u8,           // Delay Timer Register
    st: u8,           // Sound Timer Register
    screen: [[bool; 64]; 32],
}

enum PCAction {
    Next,
    Skip(u16),
    Jump(u16),
    Stay,
}

impl CHIP8 {
    pub fn new(program_bytes: std::vec::Vec<u8>) -> Self {
        let mut chip = CHIP8 {
            mem: [0; 4096],
            vx: [0; 16],
            stk: [0; 16],
            pc: 0x200,
            sp: -1,
            i: 0,
            dt: 0,
            st: 0,
            screen: [[false; 64]; 32],
        };

        let nums: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i: usize = 0;

        for num_data in nums.iter() {
            for row in num_data.iter() {
                chip.mem[i] = *row;
                i += 1;
            }
        }

        i = 0x200;

        for byte in program_bytes.iter() {
            chip.mem[i] = *byte;
            i += 0x001;
        }

        chip
    }

    fn tick_delay_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn tick_sound_timer(&mut self) {
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn stack_push(&mut self, val: u16) -> Result<(), &str> {
        if self.sp != (self.stk.len() - 1) as i16 {
            self.sp += 1;
            self.stk[self.sp as usize] = val;
            Ok(())

        } else {
            Err("Stack overflow detected.")
        }
    }

    fn stack_pop(&mut self) -> Result<u16, &str> {
        if self.sp >= 0 {
            let data = self.stk[self.sp as usize];
            self.sp -= 1;
            Ok(data)

        } else {
            Err("Stack underflow detected.")
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Result<bool, &str> {
        if x < 64 && y < 32 {
            Ok(self.screen[y][x])

        } else {
            Err("Tried to get pixel out of screen.")
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: bool) -> Result<(), &str> {
        if x < 64 && y < 32 {
            self.screen[y][x] = val;
            Ok(())

        } else {
            Err("Tried to set pixel out of screen.")
        }
    }

    fn read_opcode(&self) -> u16 {
        ((self.mem[self.pc as usize] as u16) << 8) 
        | (self.mem[(self.pc + 1) as usize] as u16)
    }

    fn exec_opcode(&mut self, opcode: u16, keys: &[bool; 16], key_pressed: Option<u8>) {
        let units = (
            ((opcode & 0xF000) >> 12) as usize,
            ((opcode & 0x0F00) >> 8) as usize,
            ((opcode & 0x00F0) >> 4) as usize,
            (opcode & 0x000F) as usize
        );

        let mut screen_changed = false;

        let pc_action: PCAction = match units {
            // 00E0 - CLS
            // Clears the screen
            (0x0, 0x0, 0xE, 0x0) => {
                self.screen = [[false; 64]; 32];
                PCAction::Next
            }

            // 00EE - RET
            // Return from a subroutine.
            // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            (0x0, 0x0, 0xE, 0xE) => {
                let pc_value = self.stack_pop().expect("Stack Error");
                PCAction::Jump(pc_value)
            }

            // 1nnn - JP addr
            // Set the program counter to nnn.
            (0x1, _, _, _) => {
                let pc_addr = opcode & 0x0FFF;
                PCAction::Jump(pc_addr)
            }

            // 2nnn - CALL addr
            // Calls subroutine from nnn
            // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            (0x2, _, _, _) => {
                self.stack_push(self.pc).expect("Stack Error");
                PCAction::Jump(opcode & 0x0FFF)
            }

            // 3xkk - SE Vx, byte
            // Skip next instruction if Vx = kk.
            (0x3, _, _, _) => {
                let vx = self.vx[units.1];
                let byte = (opcode & 0x00FF) as u8;
                if vx == byte {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // 4xkk - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            (0x4, _, _, _) => {
                let vx = self.vx[units.1];
                let byte = (opcode & 0x00FF) as u8;
                if vx != byte {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // 5xy0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            (0x5, _, _, 0x0) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                if vx == vy {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // 6xkk - LD Vx, byte
            // Set Vx = kk.
            (0x6, _, _, _) => {
                let new_val = (opcode & 0x00FF) as u8;
                self.vx[units.1] = new_val;
                PCAction::Next
            }

            // 7xkk - ADD Vx, byte
            // Set Vx = Vx + kk.
            (0x7, _, _, _) => {
                let vx_val = self.vx[units.1];
                let byte = (opcode & 0x00FF) as u8;
                self.vx[units.1] = vx_val + byte;
                PCAction::Next
            }

            // 8xy0 - LD Vx, Vy
            // Set Vx = Vy
            (0x8, _, _, 0x0) => {
                self.vx[units.1] = self.vx[units.2];
                PCAction::Next
            }

            // 8xy1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            (0x8, _, _, 0x1) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                self.vx[units.1] = vx | vy;
                PCAction::Next
            }

            // 8xy2 - AND Vx, Vy
            // Set Vx = Vx AND Vy
            (0x8, _, _, 0x2) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                self.vx[units.1] = vx & vy;
                PCAction::Next
            }

            // 8xy3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy
            (0x8, _, _, 0x3) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                self.vx[units.1] = vx ^ vy;
                PCAction::Next
            }

            // 8xy4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry.
            (0x8, _, _, 0x4) => {
                let vx = self.vx[units.1] as u16;
                let vy = self.vx[units.2] as u16;
                let addn = vx + vy;
                if addn > 0xFF {
                    self.vx[0xF as usize] = 1;
                } else {
                    self.vx[0xF as usize] = 0;
                }
                self.vx[units.1] = (addn & 0x00FF) as u8;
                PCAction::Next
            }

            // 8xy5 - SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT Borrow
            (0x8, _, _, 0x5) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                if vx > vy {
                    self.vx[0xF as usize] = 1;
                } else {
                    self.vx[0xF as usize] = 0;
                }
                self.vx[units.1] = vx.wrapping_sub(vy);
                PCAction::Next
            }

            // 8xy6 - SHR Vx {, Vy}
            // Set Vx = Vx SHR 1.
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            (0x8, _, _, 0x6) => {
                let vx = self.vx[units.1];
                self.vx[0xF as usize] = vx & 0x01;
                self.vx[units.1] = vx >> 1;
                PCAction::Next
            }

            // 8xy7 - SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow.
            (0x8, _, _, 0x7) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                if vy > vx {
                    self.vx[0xF as usize] = 1;
                } else {
                    self.vx[0xF as usize] = 0;
                }
                self.vx[units.1] = vy.wrapping_sub(vx);
                PCAction::Next
            }

            // 8xyE - SHL Vx {, Vy}
            // Set Vx = Vx SHL 1.
            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            (0x8, _, _, 0xE) => {
                let vx = self.vx[units.1];
                self.vx[0xF as usize] = vx & 0x80;
                self.vx[units.1] = vx << 1;
                PCAction::Next
            }

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            (0x9, _, _, 0x0) => {
                let vx = self.vx[units.1];
                let vy = self.vx[units.2];
                if vx != vy {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // Annn - LD I, addr
            // Set I = nnn
            (0xA, _, _, _) => {
                self.i = opcode & 0x0FFF;
                PCAction::Next
            }

            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            (0xB, _, _, _) => {
                PCAction::Jump((opcode & 0x0FFF) + self.vx[0 as usize] as u16)
            }

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                let rand_n = rng.gen_range(0, 256) as u8;
                let byte = (opcode & 0x00FF) as u8;
                self.vx[units.1] = rand_n & byte;
                PCAction::Next
            }

            /*
                Dxyn - DRW Vx, Vy, nibble
                Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                The interpreter reads n bytes from memory, starting at the address stored in I.
                These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
                Sprites are XORed onto the existing screen.
                If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                If the sprite is positioned so part of it is outside the coordinates of the display,
                it wraps around to the opposite side of the screen.

            */
            (0xD, _, _, _) => {
                let s_x = units.1;
                let s_y = units.2;
                let y_max = units.3;
                let mut collision = false;

                for y in 0..y_max {
                    let y_val = ((s_y + y) % 32) as usize;
                    let mut byte = self.mem[self.i as usize + y];

                    for x in (0..8).rev() {
                        let x_val = ((s_x + x) % 64) as usize;
                        let pix_val = (byte & 0x1) == 1;
                        let drawn = pix_val ^ self.screen[y_val][x_val];
                        if !drawn {
                            collision = true;
                        }
                        self.screen[y_val][x_val] = drawn;
                        byte >>= 1;
                    }
                }

                if collision {
                    self.vx[0xF as usize] = 1;
                } else {
                    self.vx[0xF as usize] = 0;
                }

                
                screen_changed = true;

                PCAction::Next
            }

            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            (0xE, _, 0x9, 0xE) => {
                if keys[units.1] {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            (0xE, _, 0xA, 0x1) => {
                if !keys[units.1] {
                    PCAction::Skip(1)
                } else {
                    PCAction::Next
                }
            }

            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            (0xF, _, 0x0, 0x7) => {
                self.vx[units.1] = self.dt;
                PCAction::Next
            }

            // Fx0A - LD Vx, K
            // Wait for a key press, store the value of the key in Vx.
            (0xF, _, 0x0, 0xA) => {
                match key_pressed {
                    Some(val) => {
                        self.vx[units.1] = val;
                        PCAction::Next
                    }

                    None => PCAction::Stay
                }
            }

            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.vx[units.1];
                PCAction::Next
            }

            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            (0xF, _, 0x1, 0x8) => {
                self.st = self.vx[units.1];
                PCAction::Next
            }


            _ => PCAction::Next
        };

        match pc_action {
            PCAction::Next => self.pc += 2,
            PCAction::Skip(num) => self.pc += (num + 1) * 2,
            PCAction::Jump(addr) => self.pc = addr,
            PCAction::Stay => {}
        }
    }
}