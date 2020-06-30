#![allow(dead_code)]
pub struct cpu {
    // index register
    pub I: u16,
    // program counter
    pub pc: u16,
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
    pub fn CLS(&mut self)
    {
        println!("CLear Display");
    }
    pub fn RET(&mut self )
    {
        self.pc = self.stack[self.sp];
        self.sp -= 1;

    }
    pub fn JP(&mut self, op: u16 )
    {
        self.pc = op & 0x0FFF;
    }
    pub fn CALL(&mut self, op: u16 )
    {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = op & 0x0FFF;

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
}