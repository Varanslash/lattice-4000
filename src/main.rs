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
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_math::add(registers[instr[1] as usize], value);
                i += 4;
            }
            0x02 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_math::sub(registers[instr[1] as usize], value);
                i += 4;
            }
            0x03 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_math::mul(registers[instr[1] as usize], value);
                i += 4;
            }
            0x04 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_math::div(registers[instr[1] as usize], value);
                i += 4;
            }
            0x05 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_math::lmod(registers[instr[1] as usize], value);
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
                let value = grab_addr(&memory, addr);
                flags = l16_compare::cmp(registers[instr[1] as usize], value);
                i += 4;
            }
            0x09 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_logic::land(registers[instr[1] as usize], value);
                i += 4;
            }
            0x0A => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_logic::lor(registers[instr[1] as usize], value);
                i += 4;
            }
            0x0B => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_logic::lxor(registers[instr[1] as usize], value);
                i += 4;
            }
            0x0C => {
                registers[instr[1] as usize] = l16_logic::lnot(registers[instr[1] as usize]);
                i += 4;
            }
            0x0D => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_shift::bsl(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x0E => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = l16_shift::bsr(registers[instr[1] as usize], value as u16);
                i += 4;
            }
            0x0F => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                let value = grab_addr(&memory, addr);
                registers[instr[1] as usize] = value as u16;
                i += 4;
            }
            0x10 => {
                let addr = ((instr[2] as u16) << 8) | (instr[3] as u16);
                memory[addr as usize] = registers[instr[1] as usize] >> 8 as u8; // store high byte
                memory[(addr + 1) as usize] = registers[instr[1] as usize] as u8; // store low byte
                i += 4;
            }
            0x11 => {
                let addr = ((instr[1] as u16) << 8) | (instr[2] as u16);
                i = addr as usize;
            }
            0x12 => {
                let addr = ((instr[1] as u16) << 8) | (instr[2] as u16);
                if flags[0] == 1 {
                    i = addr as usize;
                } else {
                    i += 4;
                }
            }
            0x13 => {
                let addr = ((instr[1] as u16) << 8) | (instr[2] as u16);
                if flags[0] == 0 {
                    i = addr as usize;
                } else {
                    i += 4;
                }
            }
            0x14 => {
                let addr = ((instr[1] as u16) << 8) | (instr[2] as u16);
                if flags[2] == 0 {
                    i = addr as usize;
                } else {
                    i += 4;
                }
            }
            0x15 => {
                let addr = ((instr[1] as u16) << 8) | (instr[2] as u16);
                if flags[1] == 0 {
                    i = addr as usize;
                } else {
                    i += 4;
                }
            }
            0x16 => {
                let immediate = ((instr[2] as u16) << 8) | (instr[3] as u16);
                registers[instr[1] as usize] = immediate;
                i += 4;
            }
            0x17 => {
                memory[sp as usize] = registers[instr[1] as usize] >> 8 as u8; // store high byte
                memory[(sp + 1) as usize] = registers[instr[1] as usize] as u8; // store low byte
                sp -= 2;
                i += 4;
            }
            0x18 => {
                registers[instr[1] as usize] = grab_addr(&memory, sp);
                memory[sp as usize] = 0; // clear high byte
                memory[(sp + 1) as usize] = 0; // clear low byte
                sp += 2;
                i += 4;
            }
            _ => {
                panic!("Unknown instruction: {:02X}", instr[0]);
            }
        }
    }
}

fn grab_addr(memory: &[u8], addr: u16) -> u16 {
    let low = memory[(addr + 1) as usize] as u16;
    let high = memory[addr as usize] as u16;
    return ((high << 8) | low);
}