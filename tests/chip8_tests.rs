mod tests {
    use chip8::Chip8;

    #[test]
    fn initialized_correctly() {
        let chip = Chip8::new();
        assert_eq!(chip.ram.len(), 4096);
        assert_eq!(chip.pc, 0x200);
    }

    #[test]
    fn font_loads_success() {
        let chip8 = Chip8::new();

        // validate font start and finish
        assert_eq!(
            chip8.ram[0x50], 0xF0,
            "Font starting position isn't correct"
        );
        assert_eq!(chip8.ram[0x9F], 0x80, "Font ending position isn't correct");
    }

    #[test]
    fn op_00e0_clear_screen_success() {
        let mut chip8 = Chip8::new();
        chip8.display.flip_pixel(0, 0) ;
        assert_eq!(chip8.display.get_pixel(0, 0), true);
        load_run_instruction(&mut chip8, &[0x00, 0xE0]);
        assert_eq!(false, chip8.display.get_pixel(0, 0));
    }

    #[test]
    fn op_1nnn_jump_success() {
        let mut chip8 = Chip8::new();
        assert_ne!(0x0FF, chip8.pc);
        load_run_instruction(&mut chip8, &[0x10, 0xFF]);
        assert_eq!(0x0FF, chip8.pc);
    }

    /** 6XNN */
    #[test]
    fn op_6xnn_store_register_success() {
        let mut chip8 = Chip8::new();
        let x = 0x3;
        let val = 0x12;
        chip8.var_reg[x] = 0x6;
        assert_ne!(chip8.var_reg[x], val);
        load_run_instruction(&mut chip8, &[0x63, 0x12]);
        assert_eq!(chip8.var_reg[x], val);
    }

    /** 7XNN */
    #[test]
    fn op_7xnn_add_register_success() {
        let mut chip8 = Chip8::new();
        let x = 0x3;
        let val = 0x12;
        chip8.var_reg[x] = 0x6;
        assert_ne!(chip8.var_reg[x], val);
        load_run_instruction(&mut chip8, &[0x73, 0x12]);
        assert_eq!(chip8.var_reg[x], 0x18);
    }

    /** ANNN */
    #[test]
    fn op_annn_set_index_register() {
        let mut chip8 = Chip8::new();
        assert_ne!(0x123, chip8.idx_reg);
        load_run_instruction(&mut chip8, &[0xA1, 0x23]);
        assert_eq!(chip8.idx_reg, 0x123);
    }

    /** Loads an instruction and runs a single cycle */
    fn load_run_instruction(chip8: &mut Chip8, instruction: &[u8]) {
        chip8.load_memory(chip8.pc, &instruction);
        chip8.cycle();
    }
}
