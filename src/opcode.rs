pub struct Opcode {
    pub instruction: u16,
    pub w: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}
impl Opcode {
    pub fn new(instruction: u16) -> Opcode {
        Opcode {
            instruction,
            w: ((instruction & 0xF000) >> 12) as u8,
            x: ((instruction & 0x0F00) >> 8) as u8,
            y: ((instruction & 0x00F0) >> 4) as u8,
            n: (instruction & 0x000F) as u8,
            nn: (instruction & 0x00FF) as u8,
            nnn: instruction & 0x0FFF,
        }
    }

    /** For debugging purposes - returns the instruction as hexadecimal */
    pub fn instruction_to_str(&self) -> String {
        format!("{:#06x}", self.instruction)
    }
}
