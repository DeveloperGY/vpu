pub mod mem;

use mem::MEM;

struct Instruction {
    opcode: u8,
    dest: u8,
    src: u8,
    src1: u8,
    src2: u8,
    address: u8,
    immediate: u8
}

pub struct CPU {
    regs: [i8; 4],
    ram: MEM<i8>,
    rom: MEM<u16>,
    
    ins_ptr: u8,
    equal: bool,
    running: bool
}

impl CPU {
    pub fn new(rom_capacity: u8, ram_capacity: u8) -> CPU {
        let regs: [i8; 4] = [0; 4];
        let ram: MEM<i8> = MEM::new(ram_capacity, 0);
        let rom: MEM<u16> = MEM::new(rom_capacity, 0);
        let ins_ptr: u8 = 0b00000000;
        let running: bool = true;
        let equal:bool = true;
        
        
        CPU {regs, ram, rom, ins_ptr, equal, running}
    }
    
    pub fn run(self: &mut Self) {
        while self.running
        {
            let instruction: Result<u16, ()> = self.fetch();
            let instruction: u16 = match instruction {
                Ok(i) => i,
                Err(()) => {
                    self.running = false;
                    break;
                }  
            };
            let instruction: Instruction = self.decode(instruction);
            self.execute(instruction);   
        }
    }
    
    fn fetch(self: &mut Self) -> Result<u16, ()> {
        let instruction: u16 = self.rom.arr[self.ins_ptr as usize];
        if self.ins_ptr as usize + 1 > u8::MAX as usize {
            return Err(());
        }
        self.ins_ptr += 1;
        Ok(instruction)
    }
    
    fn decode(self: &Self, instruction: u16) -> Instruction {
        let opcode: u8 = ((instruction & 0b1111000000000000) >> 12) as u8;
        let dest: u8 = ((instruction & 0b0000110000000000) >> 10) as u8;
        let src: u8 = ((instruction & 0b0000001100000000) >> 8) as u8;
        let src1: u8 = ((instruction & 0b0000000011000000) >> 6) as u8;
        let src2: u8 = ((instruction & 0b0000000000110000) >> 4) as u8;
        let address: u8 = (instruction & 0b0000000011111111) as u8;
        let immediate: u8 = (instruction & 0b0000000011111111) as u8;
        Instruction{opcode, dest, src, src1, src2, address, immediate}
    }
    
    fn execute(self: &mut Self, instruction: Instruction) {
        match instruction.opcode {
            0b0000 => (),
            0b0001 => {
                // HLT
                self.running = false;
            },
            0b0010 => {
                // ADD
                self.regs[instruction.dest as usize] = self.regs[instruction.src1 as usize].wrapping_add(self.regs[instruction.src2 as usize]);
            },
            0b0011 => {
                // SUB
                self.regs[instruction.dest as usize] = self.regs[instruction.src1 as usize].wrapping_sub(self.regs[instruction.src2 as usize]);
            },
            0b0100 => {
                // AND 
                self.regs[instruction.dest as usize] = self.regs[instruction.src1 as usize] & self.regs[instruction.src2 as usize]; 
            },
            0b0101 => {
                // OR
                self.regs[instruction.dest as usize] = self.regs[instruction.src1 as usize] | self.regs[instruction.src2 as usize]; 
            },
            0b0110 => {
                // XOR
                self.regs[instruction.dest as usize] = self.regs[instruction.src1 as usize] ^ self.regs[instruction.src2 as usize]; 
            },
            0b0111 => {
                // STR
                self.ram.arr[instruction.address as usize] = self.regs[instruction.src as usize];
            },
            0b1000 => {
                // LOD
                self.regs[instruction.dest as usize] = self.ram.arr[instruction.address as usize];
            },
            0b1001 => {
                // JPE
                if self.equal {
                    self.ins_ptr = instruction.address;
                }
            },
            0b1010 => {
                // LDI
                self.regs[instruction.dest as usize] = instruction.immediate as i8;
            },
            _ => ()
        }
    }
}