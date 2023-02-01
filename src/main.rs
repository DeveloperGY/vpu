mod cpu;

use cpu::CPU;

use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Please pass a binary file!");
        return;
    }
    
    let mut file_ptr = match fs::File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Error: Binary not found!");
            return;
        }  
    };
    
    let mut file: String = String::new();
    match file_ptr.read_to_string(&mut file) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Not valid fpu binary!");
            return;
        }
    };
    
    let mut cpu: CPU = CPU::new(255, 255);
    cpu.run();
}