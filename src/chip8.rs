pub struct CHIP8 {
    mem: [u8; 4096],  // Memory for Chip-8 (4 KB)
    vx: [u8; 16],     // General Purpose Registers (V0 - VF)
    stk: [u16; 16],   // Stack
    pc: u16,          // Program Counter
    sp: i16,          // Stack Pointer
    i: u16,           // Index Register (Used for storing memory addresses)
    dt: u8,           // Delay Timer Register
    st: u8,           // Sound Timer Register
    prog: std::vec::Vec<u16>, // Chip-8 Program,
    screen: [[bool; 64]; 32],
}

enum PCAction {
    Next,
    Skip(u16),
    Jump(u16),
}

impl CHIP8 {
    pub fn new(program_bytes: std::vec::Vec<u16>) -> Self {
        let mut chip = CHIP8 {
            mem: [0; 4096],
            vx: [0; 16],
            stk: [0; 16],
            pc: 0x200,
            sp: -1,
            i: 0,
            dt: 0,
            st: 0,
            prog: program_bytes,
            screen: [[false; 64]; 32],
        };

        let nums = [
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

        let mut i = 0;

        for num_data in nums.iter() {
            for row in num_data.iter() {
                chip.mem[i as usize] = *row as u8;
                i += 1;
            }
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
        (self.prog[self.pc as usize] << 4) | (self.prog[(self.pc + 1) as usize])
    }

    fn exec_opcode(&mut self, opcode: u16) {
        let units = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        let pc_action: PCAction = match units {
            _ => PCAction::Next,
        };

        match pc_action {
            PCAction::Next => self.pc += 2,
            PCAction::Skip(num) => self.pc += num * 2,
            PCAction::Jump(addr) => self.pc = addr,
        }
    }
}