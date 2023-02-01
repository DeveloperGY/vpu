pub mod mem;

use mem::MEM;

pub struct CPU {
    regs: [i8; 4],
    ram: MEM<i8>,
    rom: MEM<i16>,
    
    ip: u8
}

impl CPU {
    pub fn new(rom_capacity: u8, ram_capacity: u8) -> CPU {
        let regs: [i8; 4] = [0; 4];
        let ram: MEM<i8> = MEM::new(ram_capacity);
        let rom: MEM<i16> = MEM::new(rom_capacity);
        let ip: u8 = 0b00000000;
        
        CPU {regs, ram, rom, ip}
    }
    
    pub fn run(self: &mut Self) {
        let instruction: i16 = self.fetch();
        self.decode(instruction);
    }
    
    fn fetch(self: &mut Self) -> i16
    {
        let instruction: i16 = self.rom.arr[self.ip as usize];
        self.ip += 1;
        instruction
    }
    
    fn decode(self: &Self, instruction: i16) {
        let _val: i16 = instruction;
    }
}