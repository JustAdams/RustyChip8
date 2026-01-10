use chip8::Chip8;

fn main() {
    println!("Hello, Chip8!");

    let chip8 = Chip8::new();

    let mut is_running: bool = true;

    while is_running {
        is_running = false;
    }

    println!("Goodbye");
}
