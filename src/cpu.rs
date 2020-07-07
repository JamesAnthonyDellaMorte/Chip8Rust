#![allow(dead_code)]
extern crate sfml;
use sfml::{
    window::{Key},
};

pub struct cpu {
    // index register
    pub I: usize,
    // program counter
    pub pc: usize,
    // memory
    pub memory: [u8; 0x1000],
    // registers
    pub V: [u8; 0x10],
    pub  keycode: Vec<Key>,
    // stack
    pub stack: [usize; 0x10],
    // stack pointer
    pub sp: usize,
    pub hexSprites: [u8; 0x50],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub screen: [u8; 64 * 32],
    pub drawFlag: bool,


}
impl cpu {
    pub fn new() -> Self
    {
        Self {

        pc: 0x200,
        I: 0,
        sp: 0,
        keycode: vec!(Key::Num1,Key::Num2,Key::Num3,Key::Num4,
                      Key::Q,Key::W,Key::E,Key::R,
                      Key::A,Key::S,Key::D,Key::F,
                      Key::Z,Key::X,Key::C,Key::V),
        memory: [0;0x1000],
        V: [0; 0x10],
        stack: [0; 0x10],
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

        let opcode: u16 = u16::from_be_bytes([self.memory[self.pc],self.memory[self.pc + 1]]);
        let nibbles = (
            (opcode & 0xF000) >> 12 as u16,
            (opcode & 0x0F00) >> 8 as u16,
            (opcode & 0x00F0) >> 4 as u16,
            (opcode & 0x000F) as u8,
        );
      //  println!("Current Opcode: {:X}", opcode);
        // println!("PC: {:X}", self.pc);

        match nibbles

        {
            (0x00, 0x00, 0x0e, 0x00) => self.CLS(),//opcode 00E0
            (0x00, 0x00, 0x0e, 0x0e) => self.RET(), //opcode 00EE
            (0x01, _, _, _) => self.JP(opcode), //opcode 1nnn
            (0x02, _, _, _) => self.CALL(opcode),//opcode 2nnn
            (0x03, _, _, _) => self.SEVX(opcode),//opcode 3nnn
            (0x04, _, _, _) => self.SNEVX(opcode),//opcode 4nnn
            (0x05, _, _, _) => self.SE(opcode),//opcode 5nnn
            (0x06, _, _, _) => self.LDVX(opcode),//opcode 6nnn
            (0x07, _, _, _) => self.ADDVX(opcode),//opcode 7nnn
            (0x08, _, _, 0x0) => self.LD(opcode),//opcode 8nn0
            (0x08, _, _, 0x1) => self.OR(opcode),//opcode 8nn1
            (0x08, _, _, 0x2) => self.AND(opcode),//opcode 8nn2
            (0x08, _, _, 0x3) => self.XOR(opcode),//opcode 8nn3
            (0x08, _, _, 0x4) => self.ADD(opcode),//opcode 8nn4
            (0x08, _, _, 0x5) => self.SUB(opcode),//opcode 8nn5
            (0x08, _, _, 0x6) => self.SHR(opcode),//opcode 8nn6
            (0x08, _, _, 0x7) => self.SUBN(opcode),//opcode 8nn7
            (0x08, _, _, 0xE) => self.SHL(opcode),//opcode 8nnE
            (0x09, _, _, _) => self.SNE(opcode),//opcode 9nnn
            (0x0A, _, _, _) => self.LDI(opcode),//opcode Annn
            (0x0B, _, _, _) => self.JPV0(opcode),//opcode Bnnn
            (0x0C, _, _, _) => self.RND(opcode),//opcode Cnnn
            (0x0D, _, _, _) => self.DRW(opcode),//opcode Dnnn
            (0x0E, _, 0x09, 0x0E) => self.SKP(opcode),//opcode Ex9E
            (0x0E, _, 0x0A, 0x01) => self.SKNP(opcode),//opcode ExA1
            (0x0F, _, 0x00, 0x07) => self.LDVXDT(opcode),//opcode Fx07
            (0x0F, _, 0x00, 0x0A) => self.LDKEY(opcode),//opcode Fx0A
            (0x0F, _, 0x01, 0x05) => self.LDDTVX(opcode),//opcode Fx15
            (0x0F, _, 0x01, 0x08) => self.LDSTVX(opcode),//opcode Fx18
            (0x0F, _, 0x01, 0x0E) => self.ADDI(opcode),//opcode Fx1E
            (0x0F, _, 0x02, 0x09) => self.LDHEXFT(opcode),//opcode Fx29
            (0x0F, _, 0x03, 0x03) => self.LDB(opcode),//opcode Fx33
            (0x0F, _, 0x05, 0x05) => self.LDVALL(opcode),//opcode Fx55
            (0x0F, _, 0x06, 0x05) => self.LDIALL(opcode),//opcode Fx65

            _ => self.NOP(),
        }

    }
    pub fn NOP(&mut self)
    {
        self.pc += 2;
    }
    pub fn CLS(&mut self)
    {
        println!("CLear Display");
    }
    pub fn RET(&mut self )
    {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
        self.pc += 2;


    }
    pub fn JP(&mut self, op: u16 )
    //opcode 1nnn
    {
        self.pc = (op & 0x0FFF) as usize;
    }
    pub fn CALL(&mut self, op: u16 )
    //opcode 2nnn
    {

        self.stack[self.sp] = self.pc as usize;
        self.sp += 1;
        self.pc = (op & 0x0FFF) as usize;

    }
    pub fn SEVX(&mut self, op: u16)
    //opcode 3nnn
    {
        if self.V[((op & 0x0F00) >> 8) as usize] == (op & 0x00FF) as u8
    {
        self.pc += 4;
    }
    else
    {
        self.pc += 2;
    }

    }
    pub fn SNEVX(&mut self, op: u16)
    {
        //opcode 4nnn
        if self.V[((op & 0x0F00) >> 8) as usize] != (op & 0x00FF) as u8
        {
            self.pc += 4;
        }
        else
        {
            self.pc += 2;
        }

    }
    pub fn SE(&mut self, op: u16)
    {//opcode 5nnn
    if self.V[((op & 0x0F00) >> 8) as usize] == self.V[((op & 0x0F00) >> 4) as usize]
    {
        self.pc += 4;
    }
    else
    {
        self.pc += 2;
    }

    }
    pub fn LDVX(&mut self, op: u16)
    {//opcode 6nnn
        self.V[((op & 0x0F00) >> 8) as usize] = (op & 0x00FF) as u8;
        self.pc += 2;
    }
    pub fn ADDVX(&mut self, op: u16)
    {//opcode 7nnn
        self.V[((op & 0x0F00) >> 8) as usize] += (op & 0x00FF) as u8;
        self.pc += 2;
    }
    pub fn LD(&mut self, op: u16)
    {//opcode 8nn0
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn OR(&mut self, op: u16)
    {//opcode 8nn1
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] | self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn AND(&mut self, op: u16)
    {//opcode 8nn2
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] & self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn XOR(&mut self, op: u16)
    {//opcode 8nn3
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] ^ self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    pub fn ADD(&mut self, op: u16)
    {//opcode 8nn4
    let sum: u16= (self.V[((op & 0x0F00) >> 8) as usize] + self.V[((op & 0x00F0) >> 4) as usize]) as u16;
    if sum > 0xFF
    {
        self.V[0xF] = 0x1;
        self.V[((op & 0x0F00) >> 8) as usize]  = (sum & 0x00FF) as u8;

    }
    else
    {
        self.V[0xF] = 0;
        self.V[((op & 0x0F00) >> 8) as usize] = sum as u8;
    }
        self.pc += 2;
    }
    pub fn SUB(&mut self, op: u16)
    {//opcode 8nn5
        if self.V[((op & 0x0F00) >> 8) as usize] > self.V[((op & 0x00F0) >> 4) as usize]
        {
            self.V[0xF] = 0x1;
        }
        else {
            self.V[0xF] = 0x0;
        }
        self.V[((op & 0x0F00) >> 8) as usize] -= self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;

    }
    pub fn SHR(&mut self, op: u16)
    {//opcode 8nn6
        if self.V[((op & 0x0F00) >> 8) as usize] & 1 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[((op & 0x0F00) >> 8) as usize] >>= 1;
        self.pc += 2;
    }
    pub fn SUBN(&mut self, op: u16)
    {//opcode 8nn7
        if self.V[((op & 0x0F00) >> 8) as usize] < self.V[((op & 0x00F0) >> 4) as usize]
        {
            self.V[0xF] = 0x1;
        }
        else {
            self.V[0xF] = 0x0;
        }
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x00F0) >> 4) as usize] - self.V[((op & 0x0F00) >> 8) as usize];
        self.pc += 2;

    }
    pub fn SHL(&mut self, op: u16)
    {//opcode 8nnE
        if self.V[((op & 0x0F00) >> 8) as usize] >> 7 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[((op & 0x0F00) >> 8) as usize] <<= 1;
        self.pc += 2;
    }
    pub fn SNE(&mut self, op: u16)
    {//opcode 9nnn
        if self.V[((op & 0x0F00) >> 8) as usize] != self.V[((op & 0x0F00) >> 4) as usize]
        {
            self.pc += 4;
        }
        else
        {
            self.pc += 2;
        }

    }
    pub fn LDI(&mut self, op: u16)
    {//opcode Annn
        self.I = (op & 0x0FFF) as usize;
        self.pc += 2;
    }
    pub fn JPV0(&mut self, op: u16)
    {//opcode Bnnn
        self.pc = ((op & 0x0FFF) + self.V[0] as u16) as usize;
    }
    pub fn RND(&mut self, op: u16)
    {//opcode Cnnn
        let randnumber: u8 = rand::random::<u8>();
        self.V[((op & 0x0F00) >> 8) as usize] = randnumber & (op & 0x00FF) as u8;
        self.pc += 2;
    }
    pub fn DRW(&mut self, op: u16)
    {//opcode Dnnn

       let x: u16 = self.V[((op & 0x0F00) >> 8) as usize] as u16;
        let y: u16 = self.V[((op & 0x00F0) >> 4) as usize] as u16;
        let height = op & 0x000F;
        let mut pixel: u8;
        self.V[0xF] = 0;
        for yline in 0..height
        {
            pixel = self.memory[self.I + yline as usize];
            for xline in 0..8
            {
                if pixel & (0x80 >> xline) != 0
                {
                    if self.screen[(x + xline as u16 + (y + yline * 64)) as usize] == 1
                    {
                        self.V[0xF] = 1;
                    }
                    self.screen[(x + xline as u16 + (y + yline * 64)) as usize] ^= 1;
                }
            }
        }
        self.drawFlag = true;
        self.pc += 2;
    }
    pub fn SKP(&mut self, op: u16)
    {//opcode Ex9E
    if self.keycode[((op & 0x0F00) >> 8) as usize].is_pressed()
    {
        self.pc += 4;
    }
        else { self.pc += 2; }
    }
    pub fn SKNP(&mut self, op: u16)
    {//opcode ExA1
        if !self.keycode[((op & 0x0F00) >> 8) as usize].is_pressed()
        {
            self.pc += 4;
        }
        else { self.pc += 2; }
    }
    pub fn LDVXDT(&mut self, op: u16)
    {//opcode Fx07
    self.V[((op & 0x0F00) >> 8) as usize] = self.delay_timer;
    self.pc += 2;
    }
    pub fn LDKEY(&mut self, op: u16)
    {//opcode Fx0A
        'outer: loop {
            for ele in &self.keycode
            {
                if ele.is_pressed()
                {
                    self.V[((op & 0x0F00) >> 8) as usize] = self.keycode.iter().position(|&r| r == *ele).unwrap() as u8;
                    break 'outer;
                }
            }
        }
        self.pc += 2;

    }
    pub fn LDDTVX(&mut self, op: u16)
    {//opcode Fx15
        self.delay_timer = self.V[((op & 0x0F00) >> 8) as usize];
        self.pc += 2;
    }
    pub fn LDSTVX(&mut self, op: u16)
    {//opcode Fx18
        self.sound_timer = self.V[((op & 0x0F00) >> 8) as usize];
        self.pc += 2;
    }
    pub fn ADDI(&mut self, op: u16)
    {//opcode Fx1E
        self.I = (self.V[((op & 0x0F00) >> 8) as usize] + self.I as u8) as usize;
        self.pc += 2;

    }
    pub fn LDHEXFT(&mut self, op: u16) {
        //opcode 0xF029
        self.I  = (self.V[((op & 0x0F00) >> 8) as usize] * 0x5) as usize;
        self.pc += 2;
    }
    pub fn LDB(&mut self, op: u16)
    {//opcode 0xF033
    self.memory[self.I] = self.V[((op & 0x0F00) >> 8) as usize] / 100;
    self.memory[self.I + 1] = (self.V[((op & 0x0F00) >> 8) as usize]  / 10) % 10;
    self.memory[self.I + 2] = (self.V[((op & 0x0F00) >> 8) as usize] ) % 10;
    self.pc += 2;


    }
    pub fn LDVALL(&mut self, op: u16)
    {//opcode 0xF055
    for i in 0..((op & 0x0F00) >> 8)
    {
        self.memory[self.I + i as usize] = self.V[i as usize];
    }


    self.pc += 2;
    }

    pub fn LDIALL(&mut self, op: u16)
    {//opcode 0xF065
        for i in 0..((op & 0x0F00) >> 8)
        {
            self.V[i as usize] = self.memory[self.I + i as usize];
        }


        self.pc += 2;
    }
}