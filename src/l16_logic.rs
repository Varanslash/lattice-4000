pub fn lor(a: u16, b: u16) -> u16 {
    let mut res: u16 = 0;

    for i in 0..16 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res |= (1 << i),
            (0, 1) => res |= (1 << i),
            (1, 1) => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn land(a: u16, b: u16) -> u16 {
    let mut res: u16 = 0;

    for i in 0..16 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res &= !(1 << i),
            (0, 1) => res &= !(1 << i),
            (1, 1) => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn lxor(a: u16, b: u16) -> u16 {
    let mut res: u16 = 0;

    for i in 0..16 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        match (bit_a, bit_b) {
            (0, 0) => res &= !(1 << i),
            (1, 0) => res |= (1 << i),
            (0, 1) => res |= (1 << i),
            (1, 1) => res &= !(1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}

pub fn lnot(a: u16) -> u16 {
    let mut res: u16 = 0;

    for i in 0..16 {
        let bit_a = (a >> i) & 1;
        match bit_a {
            1 => res &= !(1 << i),
            0 => res |= (1 << i),
            _ => panic!("LogicError: Unknown Input")
        }
    }

    return res;
}