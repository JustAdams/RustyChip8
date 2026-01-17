use chip8::Chip8;
use chip8::rom::ROM;
use macroquad::color::{BLACK, WHITE};
use macroquad::input::KeyCode::{Escape, Left, W};
use macroquad::prelude::is_key_pressed;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{Conf, clear_background, next_frame};

const SCALE: f32 = 10.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_height: chip8::HEIGHT as i32 * SCALE as i32,
        window_width: chip8::WIDTH as i32 * SCALE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, Chip8!");

    let rom: ROM = ROM::new("ROMs/octojam1title.ch8");
    let mut chip8: Chip8 = Chip8::new();
    (&mut chip8).load_rom(rom);

    let mut is_running: bool = true;

    while is_running {
        clear_background(BLACK);

        // input
        if is_key_pressed(Escape) {
            is_running = false;
        }
        if is_key_pressed(Left) || is_key_pressed(W) {

        }

        // execute
        (&mut chip8).cycle();

        // draw display to terminal
        for row in 0..chip8::HEIGHT {
            for col in 0..chip8::WIDTH {
                let x_coord = col as f32 * SCALE;
                let y_coord = row as f32 * SCALE;
                match chip8.display.get_pixel(row, col) {
                    true => draw_rectangle(x_coord, y_coord, SCALE, SCALE, WHITE),
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
