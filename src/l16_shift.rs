pub fn bsl(na: u16) -> u16 {
    let mut res = na;

    for i in (1..16).rev() {
        let bit = (na >> (i - 1)) & 1;
        if bit == 1 {
            res |= (1 << i);
        }
        else {
            res &= !(1 << i);
        }
    }

    res &= !(1 << 0);

    return res;
}

pub fn bsr(na: u16) -> u16 {
    let mut res = na;

    for i in 0..15 {
        let bit = (na >> (i + 1)) & 1;
        if bit == 1 {
            res |= (1 << i);
        }
        else {
            res &= !(1 << i);
        }
    }

    res &= !(1 << 15);

    return res;
}