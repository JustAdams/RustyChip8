use crate::display::Display;
use crate::opcode::Opcode;
use crate::rom::ROM;

pub mod rom;

pub mod display;
mod opcode;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const SCREEN_SIZE: usize = WIDTH * HEIGHT;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub ram: [u8; 4096],
    pub display: Display,
    /** Program counter that points to the current instruction in memory */
    pub pc: u16,
    /** Index register that points to a specific location in memory */
    pub idx_reg: u16,
    /** Variable registers - 0xF is used as a flag register */
    pub var_reg: [u8; 16],
    pub call_stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
}
impl Default for Chip8 {
    fn default() -> Self {
        Chip8::new()
    }
}
impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip = Chip8 {
            ram: [0; 4096],
            display: Display::new(),
            pc: 0x200,
            idx_reg: 0,
            var_reg: [0; 16],
            call_stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        };

        for i in 0..FONT.len() {
            chip.ram[0x50 + i] = FONT[i];
        }

        chip
    }
    pub fn load_rom(&mut self, rom: ROM) {
        self.load_memory(0x200, &rom.data);
    }

    /** Performs a single fetch, decode, and execute cycle */
    pub fn cycle(&mut self) {
        // fetch instruction
        let instruction: u16 = self.fetch_instruction();

        // decode instruction
        let opcode: Opcode = Opcode::new(instruction);
        let nibbles: (u8, u8, u8, u8) = (opcode.w, opcode.x, opcode.y, opcode.n);

        // execute instruction
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.op_00e0(),
            (0x0, 0x0, 0xE, 0xE) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(opcode.nnn),
            (0x2, _, _, _) => self.op_2nnn(opcode.nnn),
            (0x3, _, _, _) => self.op_3xnn(opcode.x as usize, opcode.nn),
            (0x4, _, _, _) => self.op_4xnn(opcode.x as usize, opcode.nn),
            (0x5, _, _, _) => self.op_5xnn(opcode.x as usize, opcode.y as usize),
            (0x6, _, _, _) => self.op_6xnn(opcode.x as usize, opcode.nn),
            (0x7, _, _, _) => self.op_7xnn(opcode.x as usize, opcode.nn),
            (0x8, _, _, _) => match (nibbles.1, nibbles.2, nibbles.3) {
                // alu instructions
                (_, _, 0x0) => self.op_8xy0(opcode.x as usize, opcode.y as usize),
                (_, _, 0x1) => self.op_8xy1(opcode.x as usize, opcode.y as usize),
                (_, _, 0x2) => self.op_8xy2(opcode.x as usize, opcode.y as usize),
                (_, _, 0x3) => self.op_8xy3(opcode.x as usize, opcode.y as usize),
                (_, _, 0x4) => self.op_8xy4(opcode.x as usize, opcode.y as usize),
                (_, _, 0x5) => self.op_8xy5(opcode.x as usize, opcode.y as usize),
                (_, _, 0x6) => self.op_8xy6(opcode.x as usize, opcode.y as usize),
                (_, _, 0x7) => self.op_8xy7(opcode.x as usize, opcode.y as usize),
                (_, _, 0xE) => self.op_8xye(opcode.x as usize, opcode.y as usize),
                _ => {
                    panic!(
                        "Unsupported opcode: {opcode}",
                        opcode = opcode.instruction_to_str()
                    );
                }
            },
            (0x9, _, _, _) => self.op_9xnn(opcode.x as usize, opcode.y as usize),
            (0xA, _, _, _) => self.op_annn(opcode.nnn),
            (0xD, _, _, _) => self.op_dxyn(opcode.x as usize, opcode.y as usize, opcode.n),
            (0xF, _, _, _) => match (nibbles.1, nibbles.2, nibbles.3) {
                // timers
                (_, 0x0, 0x7) => self.op_fx07(opcode.x as usize),
                (_, 0x1, 0x5) => self.op_fx15(opcode.x as usize),
                (_, 0x5, 0x5) => self.op_fx55(opcode.x as usize),
                (_, 0x6, 0x5) => self.op_fx65(opcode.x as usize),
                _ => {
                    panic!(
                        "Unsupported opcode: {opcode}",
                        opcode = opcode.instruction_to_str()
                    );
                }
            }
                _ => {
                panic!(
                    "Unsupported opcode: {opcode}",
                    opcode = opcode.instruction_to_str()
                );
            }
        }
    }

    /** Loads a block of data into memory starting at the given position */
    pub fn load_memory(&mut self, start_pos: u16, load: &[u8]) {
        for i in 0..load.len() {
            self.ram[start_pos as usize + i] = load[i];
        }
    }

    /** Returns the instruction  sitting at the current PC location. Will increment the PC by 2. */
    fn fetch_instruction(&mut self) -> u16 {
        let mut opcode: u16 = (self.ram[self.pc as usize] as u16) << 8;
        opcode |= self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;

        opcode
    }

    /** Clear screen */
    fn op_00e0(&mut self) {
        self.display.clear();
    }

    fn op_00ee(&mut self) {
        self.pc = self.call_stack.pop().unwrap();
    }

    /** Jump - Sets the PC to NNN */
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.call_stack.push(self.pc);
        self.pc = nnn;
    }

    /** Skip conditional - Skips instruction if VX equals NN */
    fn op_3xnn(&mut self, x: usize, nn: u8) {
        if self.var_reg[x] == nn {
            self.pc += 2;
        }
    }

    /** Skip conditional - Skips instruction if VX doesn't equal NN */
    fn op_4xnn(&mut self, x: usize, nn: u8) {
        if self.var_reg[x] != nn {
            self.pc += 2;
        }
    }

    /** Skip conditional - Skips instruction if VX equals VY */
    fn op_5xnn(&mut self, x: usize, y: usize) {
        if self.var_reg[x] == self.var_reg[y] {
            self.pc += 2;
        }
    }

    /** Skip conditional - Skips instruction if VX doesn't equal VY */
    fn op_9xnn(&mut self, x: usize, y: usize) {
        if self.var_reg[x] != self.var_reg[y] {
            self.pc += 2;
        }
    }

    /** Stores number NN in register VX */
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        if x >= self.var_reg.len() {
            panic!("Invalid variable register access attempted");
        }
        self.var_reg[x] = nn;
    }

    /** Adds NN to register VX */
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        if x >= self.var_reg.len() {
            panic!("Invalid variable register access attempted");
        }
        self.var_reg[x] = self.var_reg[x].wrapping_add(nn);
    }


    /** Sets VX to the value of VY */
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.var_reg[x] = self.var_reg[y];
    }

    /** Sets VX to the OR of VX and VY */
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.var_reg[x] |= self.var_reg[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.var_reg[x] &= self.var_reg[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.var_reg[x] ^= self.var_reg[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        self.var_reg[x] = self.var_reg[x].wrapping_add(self.var_reg[y]);
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        self.var_reg[x] = self.var_reg[x].wrapping_sub(self.var_reg[y]);
    }

    /** Shift - puts VY into VX and shifts VX 1 bit to the right */
    fn op_8xy6(&mut self, x: usize, y: usize) {
        self.var_reg[0xF] = self.var_reg[y] & 0x1;
        self.var_reg[x] = self.var_reg[y] >> 1;
    }

    /** Shift - puts VY into VX and shifts VX 1 bit to the left */
    fn op_8xye(&mut self, x: usize, y: usize) {
        let val = (self.var_reg[y] >> 7) & 1;
        self.var_reg[0xF] = val;
        self.var_reg[x] = self.var_reg[y];
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        self.var_reg[y] = self.var_reg[y].wrapping_sub(self.var_reg[x]);
    }

    /** Sets index register to NNN */
    fn op_annn(&mut self, nnn: u16) {
        self.idx_reg = nnn;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) {
        let y_coord = self.var_reg[y] as usize;
        let x_coord = self.var_reg[x] as usize & (WIDTH - 1);

        self.var_reg[0xF] = 0x0;

        for row in 0..n {
            // get the Nth byte of sprite data starting from address at idx_reg
            let sprite_data: u8 = self.ram[(self.idx_reg + row as u16) as usize];
            // for each bit in sprite data...
            for col in 0..8 {
                let sprite_pixel = (sprite_data >> (7 - col)) & 0x1 == 1;
                if !sprite_pixel {
                    continue;
                }

                let x_pos = x_coord + col as usize;
                if x_pos >= WIDTH {
                    break;
                }

                let y_pos = (y_coord + row as usize) % HEIGHT;
                let curr_pixel = self.display.get_pixel(y_pos, x_pos);

                // set VF to 1 if sprite pixel and display pixel are both on
                if curr_pixel {
                    self.var_reg[0xF] = 0x1;
                }

                self.display.flip_pixel(y_pos, x_pos);
            }
        }
    }

    fn op_fx07(&mut self, x: usize) {
        self.var_reg[x] = self.delay_timer;
    }

    fn op_fx15(&mut self, x: usize) {
        self.delay_timer = self.var_reg[x];
    }

    fn op_fx55(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.ram[self.idx_reg as usize + i] = self.var_reg[i];
        }
    }

    fn op_fx65(&mut self, x: usize) {
        for i in 0..x + 1 {
            self.var_reg[i] = self.ram[self.idx_reg as usize + i];
        }
    }
}
