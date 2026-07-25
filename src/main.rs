mod l16_math;
mod l16_compare;
mod l16_logic;
mod l16_shift;

use std::fs;
use std::env;

fn main() {
    // Code Loader
    let args: Vec<String> = env::args().collect();
    let code: Vec<u8> = fs::read(args[1].clone()).expect("Unable to read file");
    let debug = args.len() > 2 && args[2] == "--debug";

    // PC
    let mut i: usize = 0; // why does every programmer use i as the pc
    let mut sp: u16 = 0xFFFF; // no, you don't get to set your own sp
    let mut memory: [u8; 65536] = [0; 65536]; // Memory: 64KB of memory, 65536 bytes flat. if you want more buy from like samsung or something
    let mut registers: [u16; 8] = [0; 8]; // Registers: [R0, R1, R2, R3, R4, R5, R6, R7] in that order, or A, B, C, W, R, X, Y, Z because register names are completely arbitrary
    let mut flags: [u16; 3] = [0; 3]; // Flags: [EQ, LT, GT] in that order because my compare function said so

    while i < code.len() {
        let instr = [code[i], code[i + 1], code[i + 2], code[i + 3]];

        match instr[0] {
            0x00 => {
                // NOP
                i += 4;
            }
            0x01 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                registers[instr[1] as usize] = l16_math::add(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x02 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                registers[instr[1] as usize] = l16_math::sub(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x03 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                registers[instr[1] as usize] = l16_math::mul(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x04 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                registers[instr[1] as usize] = l16_math::div(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x05 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                registers[instr[1] as usize] = l16_math::lmod(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x06 => {
                registers[instr[1] as usize] = l16_math::add(registers[instr[1] as usize], 1);
                i += 4;
            }
            0x07 => {
                registers[instr[1] as usize] = l16_math::sub(registers[instr[1] as usize], 1);
                i += 4;
            }
            0x08 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = memory[addr as usize];
                flags = l16_compare::compare(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            _ => {
                panic!("Unknown instruction: {:02X}", instr[0]);
            }
        }
    }
}