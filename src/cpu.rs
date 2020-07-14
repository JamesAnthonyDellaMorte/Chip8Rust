#![allow(dead_code)]
extern crate sfml;
use sfml::{
    window::{Key},

};
extern crate ears;
use ears::{Sound, AudioController};
use std::{thread, time};


pub struct cpu {
    // index register
    pub I: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 0x1000],
    // registers
    pub V: [u8; 0x10],
    pub  keycode: Vec<Key>,
    // stack pointer
    pub stack: Vec<u16>,
    pub hexSprites: [u8; 0x50],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub screen: [u8; 64 * 32],
    pub drawFlag: bool,
    sound: Sound,





}
impl cpu {
    pub fn new() -> Self
    {
        Self {

        pc: 0x200,
        I: 0,
        stack: Vec::new(),
        keycode: vec!( Key::X,Key::Num1, Key::Num2,Key::Num3,
                       Key::Q,Key::W,Key::E,Key::A,
                       Key::S,Key::D,Key::Z,Key::C,
                       Key::Num4,Key::R,Key::F,Key::V),
        memory: [0;0x1000],
        V: [0; 0x10],
        sound_timer: 0,
        delay_timer: 0,
        hexSprites: [
            0xF0, 0x90, 0x90, 0x90, 0xF0, //0
            0x20, 0x60, 0x20, 0x20, 0x70, //1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
            0x90, 0x90, 0xF0, 0x10, 0x10, //4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
            0xF0, 0x10, 0x20, 0x40, 0x40, //7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
            0xF0, 0x90, 0xF0, 0x90, 0x90, //A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
            0xF0, 0x80, 0x80, 0x80, 0xF0, //C
            0xE0, 0x90, 0x90, 0x90, 0xE0, //D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
            0xF0, 0x80, 0xF0, 0x80, 0x80  //F
            ],
            screen: [0; 64 * 32],
            drawFlag: false,
            sound: Sound::new("src/beep.wav").unwrap(),

        }

      //  for elem in self.memory.iter_mut() { *elem = 0; }
    }
    pub fn init_memory(&mut self, path: &str )
    {
        let  rom = std::fs::read(path).unwrap();
        for i in 0..self.hexSprites.len() {
            self.memory[i] = self.hexSprites[i];
        }
       for i in 0..rom.len()
       {
           self.memory[0x200 + i] = rom[i];
       }

        println!("Game loaded");



    }
    pub fn run(&mut self,)
    {

        let opcode: u16 = u16::from_be_bytes([self.memory[self.pc as usize],self.memory[self.pc as usize + 1]]);
        let nibbles = (
            (opcode & 0xF000) >> 12 as u16,
            (opcode & 0x0F00) >> 8 as u16,
            (opcode & 0x00F0) >> 4 as u16,
            (opcode & 0x000F) as u8,
        );
      //  println!("Current Opcode: {:X}", opcode);
        // println!("PC: {:X}", self.pc);
        let last_2bytes = ((nibbles.2 << 4) | nibbles.3 as u16) as u8;
        let last_3byes =(nibbles.1 << 8) | last_2bytes as u16;
         match nibbles

        {
            (0x00, 0x00, 0x0e, 0x00) => self.CLS(),//opcode 00E0
            (0x00, 0x00, 0x0e, 0x0e) => self.RET(), //opcode 00EE
            (0x01, _, _, _) => self.JP(last_3byes), //opcode 1nnn
            (0x02, _, _, _) => self.CALL(last_3byes),//opcode 2nnn
            (0x03, _, _, _) => self.SEVX(nibbles.1 as u8, last_2bytes ),//opcode 3xkk
            (0x04, _, _, _) => self.SNEVX(nibbles.1 as u8, last_2bytes ),//opcode 4xkk
            (0x05, _, _, 0x00) => self.SE(nibbles.1 as u8, nibbles.2 as u8),//opcode 5xy0
            (0x06, _, _, _) => self.LDVX(nibbles.1 as u8, last_2bytes ),//opcode 6xkk
            (0x07, _, _, _) => self.ADDVX(nibbles.1 as u8, last_2bytes),//opcode 7xkk
            (0x08, _, _, 0x0) => self.LD(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy0
            (0x08, _, _, 0x1) => self.OR(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy1
            (0x08, _, _, 0x2) => self.AND(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy2
            (0x08, _, _, 0x3) => self.XOR(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy3
            (0x08, _, _, 0x4) => self.ADD(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy4
            (0x08, _, _, 0x5) => self.SUB(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy5
            (0x08, _, _, 0x6) => self.SHR(nibbles.1 as u8),//opcode 8xy6
            (0x08, _, _, 0x7) => self.SUBN(nibbles.1 as u8, nibbles.2 as u8),//opcode 8xy7
            (0x08, _, _, 0xE) => self.SHL(nibbles.1 as u8),//opcode 8xyE
            (0x09, _, _, 0x00) => self.SNE(nibbles.1 as u8, nibbles.2 as u8),//opcode 9xy0
            (0x0A, _, _, _) => self.LDI(last_3byes),//opcode Annn
            (0x0B, _, _, _) => self.JPV0(last_3byes),//opcode Bnnn
            (0x0C, _, _, _) => self.RND(nibbles.1 as u8, last_2bytes),//opcode Cxkk
            (0x0D, _, _, _) => self.DRW(nibbles.1 as u8, nibbles.2 as u8, nibbles.3),//opcode Dxyn
            (0x0E, _, 0x09, 0x0E) => self.SKP(nibbles.1 as u8),//opcode Ex9E
            (0x0E, _, 0x0A, 0x01) => self.SKNP(nibbles.1 as u8),//opcode ExA1
            (0x0F, _, 0x00, 0x07) => self.LDVXDT(nibbles.1 as u8),//opcode Fx07
            (0x0F, _, 0x00, 0x0A) => self.LDKEY(nibbles.1 as u8),//opcode Fx0A
            (0x0F, _, 0x01, 0x05) => self.LDDTVX(nibbles.1 as u8),//opcode Fx15
            (0x0F, _, 0x01, 0x08) => self.LDSTVX(nibbles.1 as u8),//opcode Fx18
            (0x0F, _, 0x01, 0x0E) => self.ADDI(nibbles.1 as u8),//opcode Fx1E
            (0x0F, _, 0x02, 0x09) => self.LDHEXFT(nibbles.1 as u8),//opcode Fx29
            (0x0F, _, 0x03, 0x03) => self.LDB(nibbles.1 as u8),//opcode Fx33
            (0x0F, _, 0x05, 0x05) => self.LDVALL(nibbles.1 as u8),//opcode Fx55
            (0x0F, _, 0x06, 0x05) => self.LDIALL(nibbles.1 as u8),//opcode Fx65

            _ => (),
        }
        while self.delay_timer > 0
        {

            self.delay_timer -= 1;
        }
        while self.sound_timer > 0
        {

                self.sound.play();
                thread::sleep(time::Duration::from_millis(15));
                self.sound.stop();
                self.sound_timer -= 1;


        }


    }

    pub fn CLS(&mut self)
    {
        for pixel in self.screen.iter_mut() {*pixel = 0}
        self.drawFlag = true;
        self.pc += 2;

    }
    pub fn RET(&mut self )
    {
        self.pc = self.stack.remove(0);
        self.pc += 2;

    }
    pub fn JP(&mut self, nnn: u16 )
    //opcode 1nnn
    {
        self.pc = nnn;
    }
    pub fn CALL(&mut self, nnn: u16 )
    //opcode 2nnn
    {

        self.stack.push(self.pc);
        self.pc = nnn;

    }
    pub fn SEVX(&mut self, x:u8, kk:u8)
    //opcode 3xkk
    {
        if self.V[x as usize] == kk as u8
    {
        self.pc += 4;
    }
    else
    {
        self.pc += 2;
    }

    }
    pub fn SNEVX(&mut self,x: u8 , kk:u8)
    {
        //opcode 4xkk
        if self.V[x as usize] != kk
        {
            self.pc += 4;
        }
        else
        {
            self.pc += 2;
        }

    }
    pub fn SE(&mut self, x: u8, y: u8)
    {//opcode 5xy0
    if self.V[x as usize] == self.V[y as usize]
    {
        self.pc += 4;
    }
    else
    {
        self.pc += 2;
    }

    }
    pub fn LDVX(&mut self,x: u8 , kk:u8)
    {//opcode 6xkk
        self.V[x as usize] = kk;
        self.pc += 2;
    }
    pub fn ADDVX(&mut self, x: u8 , kk:u8)
    {//opcode 7xkk
        let sum = self.V[x as usize].wrapping_add(kk);
        self.V[x as usize] = sum as u8;
        self.pc += 2;
    }
    pub fn LD(&mut self, x: u8, y: u8)
    {//opcode 8xy0
        self.V[x as usize] = self.V[y as usize];
        self.pc += 2;
    }
    pub fn OR(&mut self, x: u8, y: u8)
    {//opcode 8xy1
        self.V[x as usize] |= self.V[y as usize];
        self.pc += 2;
    }
    pub fn AND(&mut self, x: u8, y: u8)
    {//opcode 8xy2
        self.V[x as usize] &= self.V[y as usize];
        self.pc += 2;
    }
    pub fn XOR(&mut self, x: u8, y: u8)
    {//opcode 8xy3
        self.V[x as usize] ^= self.V[y as usize];
        self.pc += 2;
    }

    pub fn ADD(&mut self, x: u8, y: u8)
    {//opcode 8xy4
    let sum: u16 = (self.V[x as usize] as u16) + ((self.V[y as usize]) as u16);
    if sum > 0xFF
    {
        self.V[0xF] = 0x1;
        self.V[x as usize]  = (sum & 0x00FF) as u8;

    }
    else
    {
        self.V[0xF] = 0;
        self.V[x as usize] = sum as u8;
    }
        self.pc += 2;
    }
    pub fn SUB(&mut self, x: u8, y: u8)
    {//opcode 8xy5
        if self.V[x as usize] > self.V[y as usize]
        {
            self.V[0xF] = 0x1;
        }
        else {
            self.V[0xF] = 0x0;
        }

            self.V[x as usize] =self.V[x as usize].wrapping_sub( self.V[y as usize]);


        self.pc += 2;

    }
    pub fn SHR(&mut self, x: u8)
    {//opcode 8xy6
        if self.V[x as usize] & 1 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[x as usize] >>= 1;
        self.pc += 2;
    }
    pub fn SUBN(&mut self, x: u8, y: u8)
    {//opcode 8xy7
        if self.V[x as usize] < self.V[y as usize]
        {
            self.V[0xF] = 0x1;
        }
        else {
            self.V[0xF] = 0x0;
        }
        self.V[x as usize] = self.V[y as usize].wrapping_sub(self.V[x as usize]);
        self.pc += 2;

    }
    pub fn SHL(&mut self, x: u8)
    {//opcode 8xyE
        if self.V[x as usize] >> 7 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[x as usize] <<= 1;
        self.pc += 2;
    }
    pub fn SNE(&mut self, x: u8, y: u8)
    {//opcode 9nnn
        if self.V[x as usize] != self.V[y as usize]
        {
            self.pc += 4;
        }
        else
        {
            self.pc += 2;
        }

    }
    pub fn LDI(&mut self, nnn: u16)
    {//opcode Annn
        self.I = nnn as u16;
        self.pc += 2;
    }
    pub fn JPV0(&mut self, nnn: u16)
    {//opcode Bnnn
        self.pc = nnn + self.V[0] as u16;
    }
    pub fn RND(&mut self, x: u8 , kk:u8)
    {//opcode Cnnn
        let randnumber: u8 = rand::random::<u8>();
        self.V[x as usize] = randnumber & kk as u8;
        self.pc += 2;
    }
    pub fn DRW(&mut self, x: u8, y: u8, height:u8)
    {//opcode Dnnn

       let  Vx:u16 = self.V[x as usize] as u16;
       let Vy:u16 = self.V[y as usize] as u16;

        self.V[0xF] = 0;
        for yline in 0..height
        {
            let pixel = self.memory[(self.I + yline as u16) as usize];
            for xline in 0..8
            {
                if pixel & (0x80 >> xline) != 0
                {
                    let mut idx = Vx + xline as u16 + ((Vy + yline as u16) * 64);
                    idx = if idx >= self.screen.len() as u16 {
                        64*32-1
                    } else {
                        idx
                    };

                    self.screen[idx as usize] ^= 1;
                }
            }
        }
        self.drawFlag = true;
        self.pc += 2;
    }
    pub fn SKP(&mut self, x: u8)
    {//opcode Ex9E
        if self.keycode[self.V[x as usize] as usize].is_pressed()
        {
            self.pc += 4;
        }
        else { self.pc += 2; }
    }
    pub fn SKNP(&mut self, x: u8)
    {//opcode ExA1
        if !self.keycode[self.V[x as usize] as usize].is_pressed()
        {
            self.pc += 4;
        }
        else { self.pc += 2; }
    }
    pub fn LDVXDT(&mut self, x: u8)
    {//opcode Fx07
    self.V[x as usize] = self.delay_timer;
    self.pc += 2;
    }
    pub fn LDKEY(&mut self, x: u8)
    {//opcode Fx0A
        'outer: loop {
            for ele in &self.keycode
            {
                if ele.is_pressed()
                {
                    self.V[x as usize] = self.keycode.iter().position(|&r| r == *ele).unwrap() as u8;
                    break 'outer;
                }
            }
        }
        self.pc += 2;

    }
    pub fn LDDTVX(&mut self, x: u8)
    {//opcode Fx15
        self.delay_timer = self.V[x as usize];
        self.pc += 2;
    }
    pub fn LDSTVX(&mut self, x: u8)
    {//opcode Fx18
        self.sound_timer = self.V[x as usize];
        self.pc += 2;
    }
    pub fn ADDI(&mut self, x: u8)
    {//opcode Fx1E
        self.I += self.V[x as usize] as u16;
        self.pc += 2;

    }
    pub fn LDHEXFT(&mut self, x: u8) {
        //opcode 0xF029
        self.I  = (self.V[x as usize] * 0x5) as u16;
        self.pc += 2;
    }
    pub fn LDB(&mut self, x: u8)
    {//opcode 0xF033
    self.memory[self.I as usize] = self.V[x as usize] / 100;
    self.memory[self.I as usize + 1] = (self.V[x as usize]  / 10) % 10;
    self.memory[self.I as usize + 2] = (self.V[x as usize] ) % 10;
    self.pc += 2;


    }
    pub fn LDVALL(&mut self, x: u8)
    {//opcode 0xF055
    for i in 0..=x
    {
        self.memory[(self.I + i as u16) as usize] = self.V[i as usize];
    }

       // self.I += ((op & 0x0F00) >> 8) + 1;
        self.pc += 2;
    }

    pub fn LDIALL(&mut self, x: u8)
    {//opcode 0xF065
        for i in 0..=x
        {
            self.V[i as usize] = self.memory[(self.I + i as u16)  as usize];
        }

        //self.I += ((op & 0x0F00) >> 8) + 1;
        self.pc += 2;
    }
}