extern crate minifb;

use minifb::{Key, Window, WindowOptions};

mod chip8;
use chip8::Chip8;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {

    //init window
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rust-8",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    //init
    let mut rust8 = Chip8::new();
    rust8.init();
    rust8.load_rom("rom/INVADERS");


    while window.is_open() && !window.is_key_down(Key::Escape) {

        //emulator loop
        rust8.step();

        //update graphic output
        if rust8.is_draw_flag() {
            for (_count, i) in buffer.iter_mut().enumerate() {
                *i = 0;
            }   
        }

        //update the keypad
        rust8.update_keys();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}