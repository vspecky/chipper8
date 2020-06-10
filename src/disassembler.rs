use std::vec::Vec;

pub struct Disassembler<'a> {
    program: &'a Vec<u8>,
    pc: u16
}

impl<'a> Disassembler<'a> {
    pub fn new(bytes: &'a Vec<u8>) -> Self {
        Self {
            program: bytes,
            pc: 0
        }
    }

    fn read_opcode(&mut self) -> Option<u16> {
        if self.pc as usize >= self.program.len() - 2 {
            None
        } else {
            let opcode = ((self.program[self.pc as usize] as u16) << 8)
            | ((self.program[(self.pc + 1) as usize]) as u16);
            self.pc += 2;

            Some(opcode)
        }
    }

    pub fn disassemble(&mut self) -> String {
        self.pc = 0;

        let mut code: Vec<String> = Vec::new();

        while let Some(opcode) = self.read_opcode() {
            let units = (
                ((opcode & 0xF000) >> 12) as usize,
                ((opcode & 0x0F00) >> 8) as usize,
                ((opcode & 0x00F0) >> 4) as usize,
                (opcode & 0x000F) as usize
            );

            let code_str: String = match units {
                // 00E0 - CLS
                // Clears the screen
                (0x0, 0x0, 0xE, 0x0) => {
                    String::from("CLS")
                }
    
                // 00EE - RET
                // Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                (0x0, 0x0, 0xE, 0xE) => {
                    String::from("RET")
                }
    
                // 1nnn - JP addr
                // Set the program counter to nnn.
                (0x1, _, _, _) => {
                    format!("JP {:x}", opcode & 0x0FFF)
                }
    
                // 2nnn - CALL addr
                // Calls subroutine from nnn
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                (0x2, _, _, _) => {
                    format!("CALL {:x}", opcode & 0x0FFF)
                }
    
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                (0x3, _, _, _) => {
                    let byte = (opcode & 0x00FF) as u8;
                    format!("SE {:x}, {:x} ; (V{} == {} ? skip)", units.1, byte, units.1, byte)
                }
    
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                (0x4, _, _, _) => {
                    let byte = (opcode & 0x00FF) as u8;
                    format!("SNE {:x}, {:x} ; (V{} != {} ? skip)", units.1, byte, units.1, byte)
                }
    
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                (0x5, _, _, 0x0) => {
                    format!(
                        "SE {:x}, {:x} ; (V{} == V{} ? skip)",
                        units.1, units.2, units.1, units.2
                    )
                }
    
                // 6xkk - LD Vx, byte
                // Set Vx = kk.
                (0x6, _, _, _) => {
                    let new_val = (opcode & 0x00FF) as u8;
                    format!(
                        "LD {:x}, {:x} ; (V{} = {})",
                        units.1, new_val, units.1, new_val
                    )
                }
    
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                (0x7, _, _, _) => {
                    let byte = (opcode & 0x00FF) as u8;
                    format!(
                        "ADD {:x}, {:x} ; (V{} = V{} + {})",
                        units.1, byte, units.1, units.1, byte
                    )
                }
    
                // 8xy0 - LD Vx, Vy
                // Set Vx = Vy
                (0x8, _, _, 0x0) => {
                    format!(
                        "LD {:x}, {:x} ; (V{} = V{})",
                        units.1, units.2, units.1, units.2
                    )
                }
    
                // 8xy1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                (0x8, _, _, 0x1) => {
                    format!(
                        "OR {:x}, {:x} ; (V{} = V{} OR V{})",
                        units.1, units.2, units.1, units.1, units.2
                    )
                }
    
                // 8xy2 - AND Vx, Vy
                // Set Vx = Vx AND Vy
                (0x8, _, _, 0x2) => {
                    format!(
                        "AND {:x}, {:x} ; (V{} = V{} AND V{})",
                        units.1, units.2, units.1, units.1, units.2
                    )
                }
    
                // 8xy3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy
                (0x8, _, _, 0x3) => {
                    format!(
                        "XOR {:x}, {:x} ; (V{} = V{} XOR V{})",
                        units.1, units.2, units.1, units.1, units.2
                    )
                }
    
                // 8xy4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                (0x8, _, _, 0x4) => {
                    format!(
                        "ADD {:x}, {:x} ; (V{} = V{} + V{}, VF = carry)",
                        units.1, units.2, units.1, units.1, units.2
                    )
                }
    
                // 8xy5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT Borrow
                (0x8, _, _, 0x5) => {
                    format!(
                        "SUB {:x}, {:x} ; (V{} = V{} - V{}, VF = !borrow)",
                        units.1, units.2, units.1, units.1, units.2
                    )
                }
    
                // 8xy6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                (0x8, _, _, 0x6) => {
                    format!(
                        "SHR {:x} {{, {:x}}} ; (V{} = V{} >> 1, VF = LSB)",
                        units.1, units.2, units.1, units.1
                    )
                }
    
                // 8xy7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                (0x8, _, _, 0x7) => {
                    format!(
                        "SUB {:x}, {:x} ; (V{} = V{} - V{}, VF = !borrow)",
                        units.1, units.2, units.1, units.2, units.1
                    )
                }
    
                // 8xyE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                (0x8, _, _, 0xE) => {
                    format!(
                        "SHL {:x} {{, {:x}}} ; (V{} = V{} << 1, VF = MSB)",
                        units.1, units.2, units.1, units.1
                    )
                }
    
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                (0x9, _, _, 0x0) => {
                    format!(
                        "SE {:x}, {:x} ; (V{} != V{} ? skip)",
                        units.1, units.2, units.1, units.2
                    )
                }
    
                // Annn - LD I, addr
                // Set I = nnn
                (0xA, _, _, _) => {
                    let val = opcode & 0x0FFF;
                    format!(
                        "LD I, {:x} ; (I = {})",
                        val, val
                    )
                }
    
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                (0xB, _, _, _) => {
                    let val = opcode & 0x0FFF;
                    format!(
                        "JP V0, {:x} ; (PC = V0 + {})",
                        val, val
                    )
                }
    
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                (0xC, _, _, _) => {
                    let byte = (opcode & 0x00FF) as u8;
                    format!(
                        "RND {:x}, {:x} ; (V{} = rand() AND {:x})",
                        units.1, byte, units.1, byte
                    )
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
                    format!(
                        "DRW {:x}, {:x}, {:x} ; ((V{}, V{}) -> {})",
                        units.1, units.2, units.3, units.1, units.2, units.3
                    )
                }
    
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
                (0xE, _, 0x9, 0xE) => {
                    format!(
                        "SKP {:x} ; (key(V{}) ? skip)",
                        units.1, units.1
                    )
                }
    
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed.
                (0xE, _, 0xA, 0x1) => {
                    format!(
                        "SKNP {:x} ; (!key(V{}) ? skip)",
                        units.1, units.1
                    )
                }
    
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                (0xF, _, 0x0, 0x7) => {
                    format!(
                        "LD {:x}, DT ; (V{} = DT)",
                        units.1, units.1
                    )
                }
    
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                (0xF, _, 0x0, 0xA) => {
                    format!(
                        "LD {:x}, K ; (Wait till keypress then V{} = Key)",
                        units.1, units.1
                    )
                }
    
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                (0xF, _, 0x1, 0x5) => {
                    format!(
                        "LD DT, {:x} ; (DT = V{})",
                        units.1, units.1
                    )
                }
    
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                (0xF, _, 0x1, 0x8) => {
                    format!(
                        "LD ST, {:x} ; (ST = V{})",
                        units.1, units.1
                    )
                }
    
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                (0xF, _, 0x1, 0xE) => {
                    format!(
                        "ADD I, {:x} ; (I = I + V{})",
                        units.1, units.1
                    )
                }
    
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                (0xF, _, 0x2, 0x9) => {
                    format!(
                        "LD F, {:x} ; (I = Location of sprite for V{})",
                        units.1, units.1
                    )
                }
    
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                (0xF, _, 0x3, 0x3) => {
                    format!(
                        "LD B, {:x} ; (BCD(V{}) -> (I, I+1, I+2))",
                        units.1, units.1
                    )
                }
    
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                (0xF, _, 0x5, 0x5) => {
                    format!(
                        "LD [I], {:x} ; (mem[I + x] = Vx for x in 0..{})",
                        units.1, units.1
                    )
                }
    
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at location I.
                (0xF, _, 0x6, 0x5) => {
                    format!(
                        "LD {:x}, [I] ; (Vx = mem[I + x] for x in 0..{})",
                        units.1, units.1
                    )
                }
    
                // Just go ahead if illegal opcode detected.
                _ => String::from("")
            };

            code.push(code_str);
        }

        code.join("\n")
    }
}