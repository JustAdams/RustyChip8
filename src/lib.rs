use crate::opcode::Opcode;

mod opcode;

// TODO: display stuff should be in its own module to improve extensibility
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
    pub display: [bool; SCREEN_SIZE],
    /** Program counter that points to the current instruction in memory */
    pub pc: u16,
    /** Index register that points to a specific location in memory */
    pub idx_reg: u16,
    /** Variable registers - 0xF is used as a flag register */
    pub var_reg: [u8; 16],
}
impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip =
            Chip8 {
            ram: [0; 4096],
            display: [false; 64 * 32],
            pc: 0,
            idx_reg: 0,
                var_reg: [0; 16],
        };

        for i in 0..FONT.len() {
            chip.ram[0x50 + i] = FONT[i];
        }

        chip
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
            (0x1, _, _, _) => self.op_1nnn(opcode.nnn),
            (0x6, _, _, _) => self.op_6xnn(opcode.x, opcode.nn),
            (0x7, _, _, _) => self.op_7xnn(opcode.x, opcode.nn),
            (0xA, _, _, _) => self.op_annn(opcode.nnn),
            (0xD, _, _, _) => self.op_dxyn(opcode.x, opcode.y, opcode.n),
            _ => {
                //panic!("Unsupported opcode: " + opcode.instruction_to_str());
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
        self.display = [false; SCREEN_SIZE];
    }

    /** Jump - Sets the PC to NNN */
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /** Stores number NN in register VX */
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        if x < 0 || x >= self.var_reg.len() as u8 {
            panic!("Invalid variable register access attempted");
        }
        self.var_reg[x as usize] += nn;
    }

    /** Adds NN to register VX */
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        if x < 0 || x >= self.var_reg.len() as u8 {
            panic!("Invalid variable register access attempted");
        }
        self.var_reg[x as usize] += nn;
    }

    /** Sets index register to NNN */
    fn op_annn(&mut self, nnn: u16) {
        self.idx_reg = nnn;
    }

    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) {
        let x_coord = self.var_reg[x as usize];
        let y_coord = self.var_reg[y as usize];


    }
}
