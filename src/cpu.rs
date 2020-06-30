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
    pub stack: [u16; 0x10],
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
        self.stack[self.sp] = self.pc as u16;
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

}