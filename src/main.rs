extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::env;

mod chip8;
use chip8::Chip8;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {

    // get command line arguments for path
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("no path to rom provided! (e.g. ./rust-8 rom/INVADERS)");
        return;
    }

    let rom_path = &args[1];
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
    window.limit_update_rate(None);


    //init
    let mut rust8 = Chip8::new();
    rust8.init();
    rust8.load_rom(rom_path);


    while window.is_open() && !window.is_key_down(Key::Escape) {

        //emulator loop
        rust8.step();
    
        //update graphic output
        if rust8.is_draw_flag() {
            for (count, i) in buffer.iter_mut().enumerate() {

                let x_coord = (count % WIDTH) as f64 / WIDTH as f64 * 64.0;
                let y_coord = (count / WIDTH) as f64 / HEIGHT as f64 * 32.0;

                if rust8.get_vram(x_coord as usize, y_coord as usize) {
                    *i = 0;
                }else{
                    *i = std::u32::MAX;
                }

                
            }   
                 
            rust8.reset_draw_flag();

            window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
        }else {
            window.update();
        }


        //update the keypad
        rust8.keypad[0x1] = window.is_key_down(Key::Key1) as u16;
        rust8.keypad[0x2] = window.is_key_down(Key::Key2) as u16;
        rust8.keypad[0x3] = window.is_key_down(Key::Key3) as u16;
        rust8.keypad[0xC] = window.is_key_down(Key::Key4) as u16;
        rust8.keypad[0x4] = window.is_key_down(Key::Q) as u16;
        rust8.keypad[0x5] = window.is_key_down(Key::W) as u16;
        rust8.keypad[0x6] = window.is_key_down(Key::E) as u16;
        rust8.keypad[0xD] = window.is_key_down(Key::R) as u16;
        rust8.keypad[0x7] = window.is_key_down(Key::A) as u16;
        rust8.keypad[0x8] = window.is_key_down(Key::S) as u16;
        rust8.keypad[0x9] = window.is_key_down(Key::D) as u16;
        rust8.keypad[0xE] = window.is_key_down(Key::F) as u16;
        rust8.keypad[0xA] = window.is_key_down(Key::Z) as u16;
        rust8.keypad[0x0] = window.is_key_down(Key::X) as u16;
        rust8.keypad[0xB] = window.is_key_down(Key::C) as u16;
        rust8.keypad[0xF] = window.is_key_down(Key::V) as u16;

    }
}