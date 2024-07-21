# Chip-8 Emulator in Rust

This is a simple Chip-8 emulator implemented in Rust. The Chip-8 is an interpreted programming language used in the development of early video games. This emulator allows you to run and play Chip-8 ROMs on your computer.


## Getting Started

1. **Prerequisites:**
    - Make sure you have Rust installed on your system. If not, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. **Clone the Repository:**
    ```bash
    git clone https://github.com/calalalizade/Chip8-rust
    cd chip8-emulator-rust
    ```

3. **Build and Run:**
    ```bash
    cargo build --release
    cargo run --release
    ```

    
## Controllers

The CHIP-8 system originally used a 16-key hexadecimal keypad (0-F). In the emulator, I map these keys to a modern keyboard layout for player input. The default key mapping is as follows:

| HEX Key | Keyboard Key |
| ------- | ------------ |
|   1     |      1       |
|   2     |      2       |
|   3     |      3       |
|   C     |      4       |
|   4     |      Q       |
|   5     |      W       |
|   6     |      E       |
|   D     |      R       |
|   7     |      A       |
|   8     |      S       |
|   9     |      D       |
|   E     |      F       |
|   A     |      Z       |
|   0     |      X       |
|   B     |      C       |
|   F     |      V       |

## Dependencies

- [rand](https://crates.io/crates/rand)
- [minifb](https://crates.io/crates/minifb)

## Resources

- [Chip-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Chip8 book](https://aquova.net/chip8/chip8.pdf)
