#![allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod cpu;
fn main() {
    let mut chip8 = cpu::cpu::new();
    chip8.load_rom("src/Pong.ch8");


}
