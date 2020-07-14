#![allow(non_snake_case)]
extern crate core;

#[allow(non_camel_case_types)]
mod cpu;
use std::{thread,time};
use sfml::system::{Vector2f};
use sfml::window:: {Event, Key,Style};
use sfml::graphics::{Color, Image, RenderWindow, RenderTarget, Sprite, Texture, Transformable};



fn main() {

    let mut chip8 = cpu::cpu::new();
    chip8.init_memory("src/Pong.ch8");
    // let desktop = VideoMode::desktop_mode();
    let mut window = RenderWindow::new((1280, 640),
                                       "Chip8 Emualtor",
                                       Style::CLOSE,
                                       &Default::default(),
    );


    window.set_vertical_sync_enabled(true);
    window.clear(sfml::graphics::Color::BLACK);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => {}
            }
        }
        chip8.run();

        if chip8.drawFlag == true
        {
                draw(&mut window, &chip8.screen,  chip8.drawFlag);
        }
        chip8.drawFlag = false;


        thread::sleep(time::Duration::from_millis(2));


    }
}


fn draw(window: &mut RenderWindow, graphics:&[u8], flag:  bool)
{
    let mut gfx: Vec<u8> = Vec::with_capacity(graphics.len());
    for i in 0..graphics.len() {
      let value = match graphics[i] {
            0 => &[0,0,0,0xFF], // RGBA
            _ => &[0xFF,0xFF,0xFF,0xFF] //RGBA
        };


            gfx.extend_from_slice(value);

    }

    if flag == true
    {
        let img = match Image::create_from_pixels(64, 32, &gfx) {
            Some(s) => s,
            None => panic!("Couldn't create image from pixel array")
        };
        let tex = match Texture::from_image(&img) {
            Some(s) => s,
            None => panic!("Couldn't create texture from image")
        };
        let mut sprite = Sprite::with_texture(&tex);
        sprite.scale(Vector2f::new(20f32, 20f32));
        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.display();


    }

}
