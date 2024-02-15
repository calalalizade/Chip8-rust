#![allow(unused_variables, dead_code)]

use std::fs::File;
use std::io::{ BufReader, Read };
use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;

const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;

const START_ADDR: u16 = 0x200;

const FONTS: [u8; 80] = [
    0xf0,
    0x90,
    0x90,
    0x90,
    0xf0, // 0
    0x20,
    0x60,
    0x20,
    0x20,
    0x70, // 1
    0xf0,
    0x10,
    0xf0,
    0x80,
    0xf0, // 2
    0xf0,
    0x10,
    0xf0,
    0x10,
    0xf0, // 3
    0x90,
    0x90,
    0xf0,
    0x10,
    0x10, // 4
    0xf0,
    0x80,
    0xf0,
    0x10,
    0xf0, // 5
    0xf0,
    0x80,
    0xf0,
    0x90,
    0xf0, // 6
    0xf0,
    0x10,
    0x20,
    0x40,
    0x40, // 7
    0xf0,
    0x90,
    0xf0,
    0x90,
    0xf0, // 8
    0xf0,
    0x90,
    0xf0,
    0x10,
    0xf0, // 9
    0xf0,
    0x90,
    0xf0,
    0x90,
    0x90, // A
    0xe0,
    0x90,
    0xe0,
    0x90,
    0xe0, // B
    0xf0,
    0x80,
    0x80,
    0x80,
    0xf0, // C
    0xe0,
    0x90,
    0x90,
    0x90,
    0xe0, // D
    0xf0,
    0x80,
    0xf0,
    0x80,
    0xf0, // E
    0xf0,
    0x80,
    0xf0,
    0x80,
    0x80, // F
];

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; MEMORY_SIZE],
    index: u16,
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    video: [u8; VIDEO_WIDTH * VIDEO_HEIGHT],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            registers: [0; 16],
            memory: [0; MEMORY_SIZE],
            index: 0,
            pc: START_ADDR,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            video: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
            opcode: 0,
        };
        // Store the font data in memory
        for (i, &byte) in FONTS.iter().enumerate() {
            chip8.memory[i] = byte;
        }

        chip8
    }

    pub fn convert_screen_to_buffer(&self) -> Vec<u32> {
        let white_color = 0xffffffff; // White color in u32 format
        let black_color = 0x00000000; // Black color in u32 format

        // Iterate through the Chip8 screen buffer and convert each pixel to u32 color
        let mut buffer: Vec<u32> = Vec::with_capacity(VIDEO_WIDTH * VIDEO_HEIGHT);

        for &pixel in self.video.iter() {
            // Choose color based on the pixel value
            let color = if pixel != 0 { white_color } else { black_color };

            // Push the color to the buffer
            buffer.push(color);
        }

        buffer
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut buffer: Vec<u8> = Vec::new();

        let file = File::open(filename).expect("No such file!");
        let reader = BufReader::new(file);

        for byte in reader.bytes() {
            buffer.push(byte.unwrap());
        }

        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + buffer.len();
        self.memory[start..end].copy_from_slice(&buffer);
    }

    pub fn cycle(&mut self) {
        self.fetch();
        self.decode_execute();
        self.update_timers();
    }

    pub fn set_key_pressed(&mut self, index: usize, pressed: bool) {
        self.keypad[index] = pressed;
    }

    pub fn get_display(&self) -> &[u8] {
        &self.video
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn fetch(&mut self) {
        self.opcode =
            ((self.memory[self.pc as usize] as u16) << 8) |
            (self.memory[(self.pc + 1) as usize] as u16);
        self.pc += 2;
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn decode_execute(&mut self) {
        let d1 = (self.opcode & 0xf000) >> 12;
        let d2 = (self.opcode & 0x0f00) >> 8;
        let d3 = (self.opcode & 0x00f0) >> 4;
        let d4 = self.opcode & 0x000f;

        let nnn = self.opcode & 0xfff;
        let kk = (self.opcode & 0xff) as u8;
        let x = d2 as usize;
        let y = d3 as usize;

        match (d1, d2, d3, d4) {
            // 00E0 -> CLS - Clear the display.
            (0, 0, 0xe, 0) => {
                self.video = [0; VIDEO_WIDTH * VIDEO_HEIGHT];
            }

            // 00EE -> RET - Return from a subroutine.
            (0, 0, 0xe, 0xe) => {
                self.pc = self.pop();
            }

            // 1nnn -> JP addr - Jump to location nnn.
            (1, _, _, _) => {
                self.pc = self.opcode & 0xfff;
            }

            // 2nnn -> CALL addr - Call subroutine at nnn.
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = self.opcode & 0xfff;
            }

            // 3xkk -> SE Vx, byte - Skip next instruction if Vx = kk.
            (3, _, _, _) => {
                if self.registers[x] == kk {
                    self.pc += 2;
                }
            }

            // 4xkk -> SNE Vx, byte - Skip next instruction if Vx != kk.
            (4, _, _, _) => {
                if self.registers[x] != kk {
                    self.pc += 2;
                }
            }

            // 5xy0 -> SE Vx, Vy - Skip next instruction if Vx = Vy.
            (5, _, _, _) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }

            // 6xkk -> LD Vx, byte - Set Vx = kk.
            (6, _, _, _) => {
                self.registers[x] = kk;
            }

            // 7xkk -> ADD Vx, byte - Set Vx = Vx + kk.
            (7, _, _, _) => {
                self.registers[x] = self.registers[x].wrapping_add(kk);
            }

            // 8xy0 -> LD Vx, Vy - Set Vx = Vy.
            (8, _, _, 0) => {
                self.registers[x] = self.registers[y];
            }

            // 8xy1 -> OR Vx, Vy - Set Vx = Vx OR Vy.
            (8, _, _, 1) => {
                self.registers[x] = self.registers[x] | self.registers[y];
            }

            // 8xy2 -> AND Vx, Vy - Set Vx = Vx AND Vy.
            (8, _, _, 2) => {
                self.registers[x] = self.registers[x] & self.registers[y];
            }

            // 8xy3 -> XOR Vx, Vy - Set Vx = Vx XOR Vy.
            (8, _, _, 3) => {
                self.registers[x] = self.registers[x] ^ self.registers[y];
            }

            // 8xy4 -> ADD Vx, Vy - Set Vx = Vx + Vy, set VF = carry.
            (8, _, _, 4) => {
                let vx = self.registers[x] as u16;
                let vy = self.registers[y] as u16;
                let sum = vx + vy;

                self.registers[0xf] = if sum > 0xff { 1 } else { 0 };

                self.registers[x] = sum as u8;
            }

            // 8xy5 -> SUB Vx, Vy Set Vx = Vx - Vy, set VF = NOT borrow.
            (8, _, _, 5) => {
                self.registers[0xf] = if self.registers[x] > self.registers[y] { 1 } else { 0 };

                self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);
            }

            // 8xy6 -> SHR Vx {, Vy} - Set Vx = Vx SHR 1.
            (8, _, _, 6) => {
                self.registers[0x0f] = self.registers[x] & 1;
                self.registers[x] >>= 1;
            }

            // 8xy7 -> SUBN Vx, Vy - Set Vx = Vy - Vx, set VF = NOT borrow.
            (8, _, _, 7) => {
                self.registers[0xf] = if self.registers[y] > self.registers[x] { 1 } else { 0 };

                self.registers[x] = self.registers[y].wrapping_add(self.registers[x]);
            }

            // 8xyE -> SHL Vx {, Vy} - Set Vx = Vx SHL 1.
            (8, _, _, 0xe) => {
                self.registers[0xf] = (self.registers[x] >> 7) & 1;
                self.registers[x] <<= 1;
            }

            // 9xy0 -> SNE Vx, Vy - Skip next instruction if Vx != Vy.
            (9, _, _, 0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }

            // Annn -> LD I, addr - Set I = nnn.
            (0xa, _, _, _) => {
                self.index = self.opcode & 0xfff;
            }

            // Bnnn -> JP V0, addr - Jump to location nnn + V0.
            (0xb, _, _, _) => {
                self.pc = (self.registers[0] as u16) + nnn;
            }

            // Cxkk -> RND Vx, byte - Set Vx = random byte AND kk.
            (0xc, _, _, _) => {
                let rand_byte = rand::thread_rng().gen::<u8>();
                self.registers[x] = rand_byte & kk;
            }

            // Dxyn -> DRW Vx, Vy, nibble - Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xd, _, _, _) => {
                self.registers[0xf] = 0;

                let height = d4;

                for i in 0..height {
                    let mut sprite_byte = self.memory[(self.index + i) as usize];
                    let row = ((self.registers[y] as u16) + i) % 32;

                    for j in 0..8 {
                        let sprite_pixel = (sprite_byte & 0x80) >> 7;
                        let col = (self.registers[x] + j) % 64;
                        let offset = (row * 64 + (col as u16)) as usize;

                        if sprite_pixel == 1 {
                            if self.video[offset] != 0 {
                                self.video[offset] = 0;
                                self.registers[0xf] = 1;
                            } else {
                                self.video[offset] = 1;
                            }
                        }

                        sprite_byte <<= 1;
                    }
                }
            }

            // Ex9E -> SKP Vx - Skip next instruction if key with the value of Vx is pressed.
            (0xe, _, 9, 0xe) => {
                if self.keypad[self.registers[x] as usize] {
                    self.pc += 2;
                }
            }

            // ExA1 -> SKNP Vx - Skip next instruction if key with the value of Vx is not pressed.
            (0xe, _, 0xa, 1) => {
                if !self.keypad[self.registers[x] as usize] {
                    self.pc += 2;
                }
            }

            // Fx07 -> LD Vx, DT - Set Vx = delay timer value.
            (0xf, _, 0, 7) => {
                self.registers[x] = self.delay_timer;
            }

            // Fx0A -> LD Vx, K - Wait for a key press, store the value of the key in Vx.
            (0xf, _, 0, 0xa) => {
                let mut pressed = false;
                for i in 0..self.keypad.len() {
                    if self.keypad[i] {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }

            // Fx15 -> LD DT, Vx - Set delay timer = Vx.
            (0xf, _, 1, 5) => {
                self.delay_timer = self.registers[x];
            }

            // Fx18 -> LD ST, Vx - Set sound timer = Vx.
            (0xf, _, 1, 8) => {
                self.sound_timer = self.registers[x];
            }

            // Fx1E -> ADD I, Vx - Set I = I + Vx.
            (0xf, _, 1, 0xe) => {
                self.index += self.registers[x] as u16;
            }

            // Fx29 -> LD F, Vx - Set I = location of sprite for digit Vx.
            (0xf, _, 2, 9) => {
                self.index = (self.registers[x] as u16) * 5;
            }

            // Fx33 -> LD B, Vx - Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0xf, _, 3, 3) => {
                let vx = self.registers[x];
                self.memory[self.index as usize] = vx / 100;
                self.memory[(self.index + 1) as usize] = (vx % 100) / 10;
                self.memory[(self.index + 2) as usize] = vx % 10;
            }

            // Fx55 -> LD [I], Vx - Store registers V0 through Vx in memory starting at location I.
            (0xf, _, 5, 5) => {
                for i in 0..=x {
                    self.memory[i + (self.index as usize)] = self.registers[i as usize];
                }
            }

            // Fx65 -> LD Vx, [I] - Read registers V0 through Vx from memory starting at location I.
            (0xf, _, 6, 5) => {
                for i in 0..=x {
                    self.registers[i as usize] = self.memory[i + (self.index as usize)];
                }
            }

            (_, _, _, _) => println!("Unknown instruction: {}", self.opcode),
        }
    }
}
