pub struct cpu {
    // index register
    pub I: u16,
    // program counter
    pub pc: u16,
    // memory
    pub memory: [u8; 4096],
    // registers
    pub V: [u8; 16],

    // stack
    pub stack: [u16; 16],
    // stack pointer
    pub sp: u8,
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
        memory: [0;4096],
        V: [0; 16],
        stack: [0; 16],
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
}