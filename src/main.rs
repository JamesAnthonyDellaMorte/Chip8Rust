#![allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod cpu;
fn main() {
    let mut chip8 = cpu::cpu::new();
    chip8.init_memory("src/Pong.ch8");
    loop {
        chip8.run();
    }



}
