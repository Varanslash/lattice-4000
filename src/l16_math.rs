pub fn add(na: u16, nb: u16) -> Vec<u16> {
    let mut w: u16 = 0;

    /*
        1:
            C = A & B
            W = A ^ B
        i:
            W = ((S = A ^ B) ^ C)
            C1 = A & B
            C2 = S & C
            W+ = C1 ^ C2
            CN = C1 | C2
            C = CN 
    */

    // 1st bit
    // C = A & B
    // W = A ^ B

    let a1 = (na >> 0) & 1;
    let b1 = (nb >> 0) & 1;
    let mut c = a1 & b1;
    w |= (a1 ^ b1) << 0;

    // 2nd to 15th bit
    // W = ((S = A ^ B) ^ C)
    // C1 = A & B
    // C2 = S & C
    // CN = C1 | C2
    // C = CN

    for i in 1..15 {
        let a = (na >> i) & 1;
        let b = (nb >> i) & 1;
        let s = a ^ b;
        w |= (s ^ c) << i;
        let c1 = a & b;
        let c2 = s & c;
        let cn = c1 | c2;
        c = cn;
    }

    // 16th bit
    // W = ((S = A ^ B) ^ C)

    let a15 = (na >> 15) & 1;
    let b15 = (nb >> 15) & 1;
    let s15 = a15 ^ b15;
    w |= (s15 ^ c) << 15;
    c = (a15 & b15) | (s15 & c);

    return vec![w, c];
}

pub fn sub(na: u16, nb: u16) -> Vec<u16> {
    // A + (!B + 1)

    let vnb = add(!nb, 1);
    let res = add(na, vnb[0]);
    return res;
}

pub fn mul(na: u16, nb: u16) -> u16 {
    let mut w: Vec<u16> = vec![0, 0];

    for _ in 0..nb {
        w = add(w[0], na);
    }

    return w[0];
}

pub fn div(mut na: u16, nb: u16) -> u16 {
    let mut w: Vec<u16> = vec![0, 0];

    while na >= nb {
        w = add(w[0], 1);
        let vna = sub(na, nb);
        na = vna[0];
    }

    return w[0];
}

pub fn lmod(mut na: u16, nb: u16) -> u16 {
    while na >= nb {
        let vna = sub(na, nb);
        na = vna[0];
    }

    return na;
}