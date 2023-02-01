mod cpu;

use cpu::CPU;

// use std::env::Args; For when its time to load the binary

fn main() {
    let mut cpu: CPU = CPU::new(255, 255);
    cpu.run();
}
