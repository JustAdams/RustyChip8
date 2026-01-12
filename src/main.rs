use chip8::Chip8;
use chip8::rom::ROM;
use macroquad::color::{BLACK, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::window::{Conf, clear_background, next_frame};

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_height: 320,
        window_width: 640,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, Chip8!");

    let rom: ROM = ROM::new("ROMs/IBM Logo.ch8");
    let mut chip8: Chip8 = Chip8::new();
    (&mut chip8).load_rom(rom);

    let is_running: bool = true;

    while is_running {
        clear_background(BLACK);

        (&mut chip8).cycle();

        // draw display to terminal
        for row in 0..chip8::HEIGHT {
            for col in 0..chip8::WIDTH {
                let x_coord = col as f32 * 10.0;
                let y_coord = row as f32 * 10.0;
                match chip8.display.get_pixel(row, col) {
                    true => draw_rectangle(x_coord, y_coord, 10.0, 10.0, WHITE),
                    false => {
                        // Not drawing a black square because the screen is set black each loop
                    }
                }
            }
        }

        next_frame().await
    }

    println!("Goodbye");
}
