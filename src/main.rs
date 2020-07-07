#![allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod cpu;
use std::{thread,time};
use sfml::system::{Vector2f};
use sfml::window:: {Event, Key,Style};
use sfml::graphics::{RenderWindow, RenderTarget, Transformable, Shape};



fn main() {

    let mut chip8 = cpu::cpu::new();
    chip8.init_memory("src/Pong.ch8");
    // let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new((640, 320),
                                       "Chip8 Emualtor",
                                       Style::CLOSE,
                                       &Default::default(),
    );


    window.set_vertical_sync_enabled(true);
    while window.is_open() {
        window.clear(sfml::graphics::Color::BLACK);
        chip8.run();
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
            if chip8.drawFlag == true
            {
                let mut ypoint: u16 = 0;
                let mut ycounter = 0;
                for i in 0..chip8.screen.len()
                {
                    let xpoint: u16 = (i % 64) as u16;
                    if chip8.screen[i] == 1
                    {
                        let mut sprite = sfml::graphics::RectangleShape::new();
                        sprite.set_position(Vector2f::new((xpoint * 10) as f32, (ypoint * 10) as f32));
                        sprite.set_fill_color(sfml::graphics::Color::WHITE);
                        window.draw(&sprite);
                    } else {
                        let mut sprite = sfml::graphics::RectangleShape::new();
                        sprite.set_position(Vector2f::new((xpoint * 10) as f32, (ypoint * 10) as f32));
                        sprite.set_fill_color(sfml::graphics::Color::BLACK);
                        window.draw(&sprite);
                    }
                    ycounter += 1;
                    if ycounter == 64
                    {
                        ypoint += 1;
                        ycounter = 0
                    }
                }
                chip8.drawFlag = false;
                thread::sleep(time::Duration::from_micros(1200));
            }
            window.display();
        }
    }
}
