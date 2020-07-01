#![allow(dead_code)]


pub struct cpu {
    // index register
    pub I: u16,
    // program counter
    pub pc: usize,
    // memory
    pub memory: [u8; 0x1000],
    // registers
    pub V: [u8; 0x10],

    // stack
    pub stack: [usize; 0x10],
    // stack pointer
    pub sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,


}
impl cpu {
    pub fn new() -> Self
    {
        Self {

        pc: 0x200,
        I: 0,
        sp: 0,
        memory: [0;0x1000],
        V: [0; 0x10],
        stack: [0; 0x10],
        sound_timer: 0,
        delay_timer: 0,
        }
      //  for elem in self.memory.iter_mut() { *elem = 0; }
    }
    pub fn load_rom(&mut self, path: &str )
    {
        let  rom = std::fs::read(path).unwrap();
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
        match nibbles
        {
            (0x00, 0x00, 0x0e, 0x00) => self.CLS(),
            (0x00, 0x00, 0x0e, 0x0e) => self.RET(),
            (0x01, _, _, _) => self.JP(opcode),
            (0x02, _, _, _) => self.CALL(opcode),
            (0x03, _, _, _) => self.SEVX(opcode),
            (0x04, _, _, _) => self.SNEVX(opcode),
            (0x05, _, _, _) => self.SE(opcode),
            (0x06, _, _, _) => self.LDVX(opcode),
            (0x07, _, _, _) => self.ADDVX(opcode),
            (0x08, _, _, 0x0) => self.LD(opcode),
            (0x08, _, _, 0x1) => self.OR(opcode),
            (0x08, _, _, 0x2) => self.AND(opcode),
            (0x08, _, _, 0x3) => self.XOR(opcode),
            (0x08, _, _, 0x4) => self.ADD(opcode),
            (0x08, _, _, 0x5) => self.SUB(opcode),
            (0x08, _, _, 0x6) => self.SHR(opcode),
            (0x08, _, _, 0x7) => self.SUBN(opcode),
            (0x08, _, _, 0xE) => self.SHL(opcode),
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
        self.pc = self.stack[self.sp] as usize;
        self.sp -= 1;

    }
    pub fn JP(&mut self, op: u16 )
    {
        self.pc = (op & 0x0FFF) as usize;
    }
    pub fn CALL(&mut self, op: u16 )
    {
        self.stack[self.sp] = self.pc as usize;
        self.sp += 1;
        self.pc = (op & 0x0FFF) as usize;

    }
    pub fn SEVX(&mut self, op: u16)
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
    {
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
    {
        self.V[((op & 0x0F00) >> 8) as usize] = (op & 0x00FF) as u8;
        self.pc += 2;
    }
    pub fn ADDVX(&mut self, op: u16)
    {
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] + (op & 0x00FF) as u8;
        self.pc += 2;
    }
    pub fn LD(&mut self, op: u16)
    {
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn OR(&mut self, op: u16)
    {
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] | self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn AND(&mut self, op: u16)
    {
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] & self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }
    pub fn XOR(&mut self, op: u16)
    {
        self.V[((op & 0x0F00) >> 8) as usize] = self.V[((op & 0x0F00) >> 8) as usize] ^ self.V[((op & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    pub fn ADD(&mut self, op: u16)
    {
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
    {
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
    {
        if self.V[((op & 0x0F00) >> 8) as usize] & 1 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[((op & 0x0F00) >> 8) as usize] >>= 1;
        self.pc += 2;
    }
    pub fn SUBN(&mut self, op: u16)
    {
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
    {
        if self.V[((op & 0x0F00) >> 8) as usize] >> 7 == 1
        {
            self.V[0xF] = 0x1;
        }
        else {  self.V[0xF] = 0x0; }
        self.V[((op & 0x0F00) >> 8) as usize] <<= 1;
        self.pc += 2;
    }

}