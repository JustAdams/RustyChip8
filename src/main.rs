use chip8::Chip8;
use chip8::rom::ROM;
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, is_key_down};
use macroquad::prelude::is_key_pressed;
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{Conf, clear_background, next_frame};

const SCALE: f32 = 10.0;
const GAME_HEIGHT: f32 = chip8::HEIGHT as f32 * SCALE as f32;
const GAME_WIDTH: f32 = chip8::WIDTH as f32 * SCALE as f32;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8".to_owned(),
        window_height: GAME_HEIGHT as i32,
        window_width: GAME_WIDTH as i32 + 250,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let debug_mode: bool = true;
    let mut opcode_stack: Vec<u16> = vec![];

    let rom: ROM = ROM::new("ROMs/octojam1title.ch8");
    let mut chip8: Chip8 = Chip8::new();
    (&mut chip8).load_rom(rom);

    let mut is_running: bool = true;

    let mut curr_timer = 0.0;
    let timer_update = 60.0;

    while is_running {
        clear_background(BLACK);

        // input
        if is_key_down(KeyCode::Escape) {
            is_running = false;
        }
        let mut curr_key: Option<u8> = None;

        if is_key_down(KeyCode::Key2) {
            curr_key = Some(0x2);
        }
        if is_key_down(KeyCode::W) {
            curr_key = Some(0x5);
        }
        if is_key_down(KeyCode::S) {
            curr_key = Some(0x8);
        }

        // update timers by 60Hz
        if curr_timer < timer_update {
            curr_timer += get_frame_time();
            chip8.decrement_timers(1);
        } else {
            curr_timer = 0.0;
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

        // draw debug details
        draw_line(GAME_WIDTH, 0.0, GAME_WIDTH, GAME_HEIGHT, 2.0, WHITE);

        if debug_mode {
            // current key input
            let curr_input = curr_key.unwrap_or_default();
            draw_text(
                format!("Current Input: {:#02x}", curr_input).as_str(),
                GAME_WIDTH + 10.0,
                25.0,
                25.0,
                WHITE,
            );
        }

        next_frame().await
    }

    println!("Goodbye");
}
