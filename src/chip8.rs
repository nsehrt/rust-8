extern crate std;
extern crate rand;

use rand::*;
use std::num::Wrapping;

pub struct Chip8{

    memory: [u8; 4096],
    vram: [u8; 64*32],
    registers: [u8; 16],
    stack: [u16; 16],
    keypad: [u16; 16],

    opcode: u16,
    pc: u16,
    sp: u16,
    index_reg: u16,

    delay_timer: u8,
    sound_timer: u8,

    draw_flag: bool,
    randomizer: rand::rngs::ThreadRng,
}



impl Chip8{

    pub fn new() -> Chip8{
        Chip8{
            memory: [0; 4096],
            vram: [0; 64*32],
            registers: [0; 16],
            stack: [0; 16],
            keypad: [0; 16],

            opcode: 0,
            pc: 0,
            sp: 0,
            index_reg: 0,

            delay_timer: 0,
            sound_timer: 0,

            draw_flag: false,
            randomizer: rand::thread_rng(),
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
        self.draw_flag = true;
    }

    pub fn load_rom(&mut self, rom: &str) {

        let bytes = std::fs::read(rom).expect("failed to load rom!");
        self.memory[0x200..(0x200+bytes.len())].clone_from_slice(&bytes[..]);
    }


    pub fn step(&mut self) {
        // fetch op code
        self.opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        // decode and execute op code
        match self.opcode & 0xF000 {

            0x0000 => {

                match self.opcode & 0x000F {

                    0x0000 => { // clear the screen
                        for i in 0..2048{
                            self.vram[i] = 0;
                        }
                        self.draw_flag = true;
                        self.pc += 2;
                    },

                    0x000E => { // return from subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize];
                        self.pc += 2;
                    },

                    _ => println!("unknown opcode: {:#02x}", self.opcode),
                }


            },

            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            },

            0x2000 => { // call subroutine at 0xxx
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            },

            0x3000 => { // skips next instruction if VX == NN
                if self.registers[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8{
                    self.pc += 4;
                }else{
                    self.pc += 2;
                }
            },

            0x4000 => { // same as 0x3000 only inverse
                if self.registers[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8{
                    self.pc += 4;
                }else{
                    self.pc += 2;
                }
            },

            0x5000 => { // same as 0x3000 but comparison between VX and VY
                if self.registers[((self.opcode & 0x0F00) >> 8) as usize] == self.registers[((self.opcode & 0x00F0) >> 4) as usize]{
                    self.pc += 4;
                }else{
                    self.pc += 2;
                }
            },

            0x6000 => { // set VX to XX
                self.registers[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            },

            0x7000 => { // add XX to VX
                self.registers[((self.opcode & 0x0F00) >> 8) as usize] += (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            },

            0x8000 => {

                match self.opcode & 0x000F {

                    0x0000 => { // set VX to value of VY
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] = self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0001 => { // set VX to VX | VY
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] |= self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0002 => { // set VX to VX & VY
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] &= self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0003 => { // set VX to VX ^ VY
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] ^= self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0004 => { // add VY to VX. carry flag is set if necessary
                        if self.registers[((self.opcode & 0x00F0) >> 4) as usize] > (0xFF - self.registers[((self.opcode & 0x0F00) >> 8) as usize]){
                            self.registers[0xF] = 1;
                        }else{
                            self.registers[0xF] = 0;
                        }
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] += self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    },

                    0x0005 => { // subtract VY from VX. carry flag used as borrow indicator
                        if self.registers[((self.opcode & 0x00F0) >> 4) as usize] <= self.registers[((self.opcode & 0x0F00) >> 8) as usize]{
                            self.registers[0xF] = 1;
                        }else{
                            self.registers[0xF] = 0;
                        }
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] -= self.registers[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;  
                    },

                    0x0006 => { // shift VX right by one. 
                        self.registers[0xF] = self.registers[((self.opcode & 0x0F00) >> 8) as usize] &0x1;
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] >>= 1;
                        self.pc += 2;
                    },

                    0x0007 => { // set VX to VY - VX. carry flag used as borrow indicator
                        if self.registers[((self.opcode & 0x0F00) >> 8) as usize] > self.registers[((self.opcode & 0x00F0) >> 4) as usize] {
                            self.registers[0xF] = 0;
                        }else{
                            self.registers[0xF] = 1;
                        }
                        self.pc += 2;
                    },

                    0x000E => { // shift vx left by one
                        self.registers[0xF] = self.registers[((self.opcode & 0x0F00) >> 8) as usize] >> 7;
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] <<= 1;
                        self.pc += 2;
                    },

                    _ => println!("unknown opcode: {:#02x}", self.opcode),
                }

            },

            0x9000 => { // skip next instruction if VX != VY
                if self.registers[((self.opcode & 0x0F00) >> 8) as usize] != self.registers[((self.opcode & 0x00F0) >> 4) as usize]{
                    self.pc += 4;
                }else{
                    self.pc += 2;
                }
            },

            0xA000 => { // index reg to XXX
                self.index_reg = self.opcode & 0x0FFF;
                self.pc += 2;
            },

            0xB000 => { // jump to address XXX + V0
                self.pc = (self.opcode & 0x0FFF) + self.registers[0x0] as u16;
            },

            0xC000 => { // set VX to a random number & XX
                self.registers[((self.opcode & 0x0F00) >> 8) as usize] = (self.randomizer.next_u32() % 0xFF) as u8 & (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            },

            0xD000 => { // D X Y N
                        // draw a sprite at VX, VY with a width of 8 pixels and a height of N pixels.
                let x = self.registers[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                let y = self.registers[((self.opcode & 0x00F0) >> 4) as usize] as u16;
                let height = self.opcode & 0x000F;
                let mut pixel;

                self.registers[0xF] = 0;

                for i in 0..height {
                    pixel = self.memory[(self.index_reg + i) as usize];

                    for j in 0..8{

                        if (pixel & (0x80 >> j)) != 0{
                            let wrap_1 = Wrapping(y as u16 + i);
                            let wrap_2 = Wrapping(64 as u16);
                            let index = x + j + (wrap_1 * wrap_2).0;

                            if self.vram[index as usize] == 1 {
                                self.registers[0xF] = 1;
                            }
                            self.vram[index as usize] ^= 1;
                        }
                    }

                }

                self.draw_flag = true;
                self.pc += 2;
            },

            0xE000 => {

                match self.opcode & 0x00FF {

                    0x009E => { // skip next instruction if key stored in vx is pressed
                        if self.registers[((self.opcode & 0x0F00) >> 8) as usize] != 0{
                            self.pc += 4;
                        }else{
                            self.pc += 2;
                        }
                    },

                    0x00A1 => { // skip next instruction if key stored in vx is not pressed
                        if self.registers[((self.opcode & 0x0F00) >> 8) as usize] == 0{
                            self.pc += 4;
                        }else{
                            self.pc += 2;
                        }
                    },

                    _ => println!("unknown opcode: {:#02x}", self.opcode),
                }

            },

            0xF000 => {

                match self.opcode & 0x00FF {

                    0x0007 => { // set VX to value of delay timer
                        self.registers[((self.opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                        self.pc += 2;
                    },

                    0x000A => {
                        let mut key_press = false;

                        for i in 0..16{
                            if self.keypad[i] != 0 {
                                self.registers[((self.opcode & 0x0F00) >> 8) as usize] = i as u8;
                                key_press = true;
                            }
                        }

                        if !key_press {
                            return;
                        }

                        self.pc += 2;
                    },

                    0x0015 => { // set delay timer to VX
                        self.delay_timer = self.registers[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    },

                    0x0018 => { // set sound timer to VX
                        self.sound_timer = self.registers[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    },

                    0x001E => { // add VX to index register
                        if self.index_reg + self.registers[((self.opcode & 0x0F00) >> 8) as usize] as u16 > 0xFFF {
                            self.registers[0xF] = 1;
                        }else{
                            self.registers[0xF] = 0;
                        }

                        self.index_reg += self.registers[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                        self.pc += 2;
                    },

                    0x0029 => { //set index register to location of the character VX in the font rom
                        self.index_reg = self.registers[((self.opcode & 0x0F00) >> 8) as usize] as u16 * 0x5;
                        self.pc += 2;
                    },

                    0x0033 => { // store bin coded representation of VX at index_register (+1, +2)
                        self.memory[self.index_reg as usize] = self.registers[((self.opcode & 0x0F00) >> 8) as usize] / 100;
                        self.memory[self.index_reg as usize + 1] = (self.registers[((self.opcode & 0x0F00) >> 8) as usize] / 10) % 10;
                        self.memory[self.index_reg as usize + 2] = (self.registers[((self.opcode & 0x0F00) >> 8) as usize] % 100) % 10;
                        self.pc += 2;
                    },

                    0x0055 => { // store V0 to VX in memor stating with index_register
                        for i in 0..((self.opcode & 0x0F00) >> 8) {
                            self.memory[(self.index_reg + i) as usize] = self.registers[i as usize];
                        }

                        self.index_reg = ((self.opcode & 0x0F00) >> 8) + 1; 
                        self.pc += 2;
                    },

                    0x0065 => { // fill V0 to VX with values from memory starting on index_register
                        for i in 0..((self.opcode & 0x0F00) >> 8) {
                            self.registers[i as usize] = self.memory[(self.index_reg + i) as usize];
                        }

                        self.index_reg = ((self.opcode & 0x0F00) >> 8) + 1; 
                        self.pc += 2;
                    },

                    _ => println!("unknown opcode: {:#02x}", self.opcode),

                }

            }

            _ => println!("unknown opcode: {:#02x}", self.opcode),
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
        self.draw_flag
    }

    pub fn update_keys(&mut self) {


    }

    pub fn get_vram(&self, x: usize, y: usize) -> bool {
        let mut index = y * 64 + x;
        if index > 2047 {
            index = 2047;
        }
        self.vram[index] == 0
    }

}