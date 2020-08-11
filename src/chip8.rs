extern crate std;

use std::fs::File;
use std::io::Read;

pub struct Chip8{

    memory: [u8; 4096],
    vram: [u8; 64*32],
    registers: [u16; 16],
    stack: [u16; 16],
    keypad: [u16; 16],

    current_opcode: u16,
    pc: u16,
    sp: u16,
    index_reg: u16,

    delay_timer: u8,
    sound_timer: u8,

}



impl Chip8{

    pub fn new() -> Chip8{
        Chip8{
            memory: [0; 4096],
            vram: [0; 64*32],
            registers: [0; 16],
            stack: [0; 16],
            keypad: [0; 16],

            current_opcode: 0,
            pc: 0,
            sp: 0,
            index_reg: 0,

            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn init(&mut self) {
        // rom starts at 0x200
        self.pc = 0x200;

        // load font
        let fontset: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        self.memory[0x50..160].clone_from_slice(&fontset[..80]); // maybe wrong address
    }

    pub fn load_rom(&mut self, rom: &str) {

        let bytes = std::fs::read(rom).expect("failed to load rom!");
        self.memory[0x200..(0x200+bytes.len())].clone_from_slice(&bytes[..]);
    }


    pub fn step(&mut self) {
        // fetch op code
        self.current_opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        // decode and execute op code
        match self.current_opcode {
            0xA000 => {
                self.index_reg = self.current_opcode & 0x0FFF;
                self.pc += 2;
            },
            _ => println!("unknown op code: {:#02x}", self.current_opcode),
        }

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("Beeeeep!");
            }
            self.sound_timer -= 1;
        }

    }

    pub fn is_draw_flag(&self) -> bool {
        true
    }

    pub fn update_keys(&mut self) {


    }

}