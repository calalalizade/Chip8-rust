#![allow(unused_variables, dead_code)]

mod processor;
use processor::Chip8;

use minifb::{ Key, Window, WindowOptions };
const WINDOW_WIDTH: usize = 64;
const WINDOW_HEIGHT: usize = 32;

fn main() {
    let mut chip8 = Chip8::new();

    chip8.load_rom("ibm.ch8");

    let mut window = Window::new("CHIP8", WINDOW_WIDTH, WINDOW_HEIGHT, WindowOptions {
        scale: minifb::Scale::X8,
        scale_mode: minifb::ScaleMode::AspectRatioStretch,
        ..WindowOptions::default()
    }).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(1)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Iterate over the keys and handle them
        for key in window.get_keys() {
            match to_chip8_key(key) {
                Some(chip8_code) => {
                    if window.is_key_down(key) {
                        chip8.set_key_pressed(chip8_code, true);
                    } else {
                        chip8.set_key_pressed(chip8_code, false);
                    }
                }
                None => (),
            }
        }

        chip8.cycle();

        // We unwrap here as we want this code to exit if it fails.
        window
            .update_with_buffer(&chip8.convert_screen_to_buffer(), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn to_chip8_key(key: Key) -> Option<usize> {
    match key {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xc),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xd),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xe),
        Key::Z => Some(0xa),
        Key::X => Some(0x0),
        Key::C => Some(0xb),
        Key::V => Some(0xf),
        _ => None,
    }
}
