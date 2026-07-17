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
    let mut labels: HashMap<String, u8> = HashMap::new();
    let mut offset: u8 = 0;
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() == 0 {
            continue;
        }
        match instruction[0].to_uppercase().as_str() {
            ".LABEL" => {
                labels.insert(instruction[1].to_string(), offset);
            }
            _ => {
                offset += 2;
            }
        }
    }
    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() == 0 {
            continue;
        }
        match instruction[0].to_uppercase().as_str() {
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
        }
    }
    return bytes;
}