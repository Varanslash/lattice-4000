use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(args[1].clone()).expect("Unable to read file");
    let bytes = assemble(code);
    fs::write(args[2].clone(), bytes).expect("Unable to write file");
}

fn assemble(code: String) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut labels: HashMap<String, u16> = HashMap::new();
    let mut offset: u16 = 0;
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() == 0 {
            continue;
        }
        match instruction[0].to_uppercase().as_str() {
            ".LABEL" => {
                labels.insert(instruction[1].to_string(), offset);
            }
            ";" => {
                continue;
            }
            _ => {
                offset += 4;
            }
        }
    }
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.is_empty() | instruction[0].starts_with(";") | instruction[0].starts_with(".LABEL") {
            continue;
        }
        match instruction[0].to_uppercase().as_str() {
            "NOP" => {
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "ADD" => {
                bytes.push(0x01);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "SUB" => {
                bytes.push(0x02);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "MUL" => {
                bytes.push(0x03);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "DIV" => {
                bytes.push(0x04);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "LMOD" => {
                bytes.push(0x05);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "INC" => {
                bytes.push(0x06);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "DEC" => {
                bytes.push(0x07);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "CMP" => {
                bytes.push(0x08);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "AND" => {
                bytes.push(0x09);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "OR" => {
                bytes.push(0x0A);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "XOR" => {
                bytes.push(0x0B);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "NOT" => {
                bytes.push(0x0C);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "BSL" => {
                bytes.push(0x0D);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "BSR" => {
                bytes.push(0x0E);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "LD" => {
                bytes.push(0x0F);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "ST" => {
                bytes.push(0x10);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let addr = instruction[2].parse::<u16>().unwrap();
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
            }
            "BRA" => {
                bytes.push(0x11);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "BEQ" => {
                bytes.push(0x12);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "BNE" => {
                bytes.push(0x13);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "BGE" => {
                bytes.push(0x14);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "BLT" => {
                bytes.push(0x15);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "LDI" => {
                bytes.push(0x16);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                let immediate = instruction[2].parse::<u16>().unwrap();
                bytes.push((immediate >> 8) as u8);
                bytes.push(immediate as u8);
            }
            "PUSH" => {
                bytes.push(0x17);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "POP" => {
                bytes.push(0x18);
                bytes.push(instruction[1].parse::<u8>().unwrap());
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "CALL" => {
                bytes.push(0x19);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "CEQ" => {
                bytes.push(0x1A);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "CNE" => {
                bytes.push(0x1B);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "CGE" => {
                bytes.push(0x1C);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "CLT" => {
                bytes.push(0x1D);
                let addr = match labels.get(instruction[1]) {
                    Some(&addr) => addr,
                    None => instruction[1].parse::<u16>().unwrap(),
                };
                bytes.push((addr >> 8) as u8);
                bytes.push(addr as u8);
                bytes.push(0x00);
            }
            "RET" => {
                bytes.push(0x1E);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            "HLT" => {
                bytes.push(0x1F);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes.push(0x00);
            }
            _ => {
                panic!("Unknown instruction: {}", instruction[0]);
            }
        }
    }
    return bytes;
}