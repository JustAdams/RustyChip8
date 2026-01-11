use chip8::Chip8;
use chip8::rom::ROM;

fn main() {
    println!("Hello, Chip8!");

    let rom: ROM = ROM::new("ROMs/test_opcode.ch8");
    let mut chip8: Chip8 = Chip8::new();
    (&mut chip8).load_rom(rom);

    let mut is_running: bool = true;

    while is_running {
        (&mut chip8).cycle();

        // draw display to terminal
        for row in 0..chip8::HEIGHT {
            for col in 0..chip8::WIDTH {
                if chip8.display[row][col] {
                    print!("* ");
                } else {
                    print!("  ");
                }
            }
            print!("\n");
        }
    }

    println!("Goodbye");
}
