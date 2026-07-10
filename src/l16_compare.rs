use crate::l16_math;

pub fn cmp(a: u16, b: u16) -> Vec<u16> {
    // Declare flags
    let mut feq = 0;
    let mut flt = 0;
    let mut fgt = 0;

    // Subtract a and b, and set flags based on the result
    let mut sres: Vec<u16> = Vec::new();
    sres = l16_math::sub(a, b);

    // Flag Setting
    fgt = sres[1]; // Overflow is a direct result of borrow
    flt = match fgt { // Underflow is !Overflow
        0 => 1,
        _ => 0,
    };
    feq = match sres[0] { // If the result is zero, then a and b are equal
        0 => 1,
        _ => 0,
    };

    return vec![feq, flt, fgt];
}