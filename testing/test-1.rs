mod l16_shift;
mod l16_logic;
mod l16_math;
mod l16_compare;

fn main() {
    let a: u16 = 0b0000000000000010; // 2 in binary
    let b: u16 = 0b0000000000000011; // 3 in binary

    let res = l16_math::add(a, b);
    println!("Add Result: {:016b}, Carry: {}", res[0], res[1]); // Expected output: 0000000000000101, Carry: 0

    let res = l16_math::sub(a, b);
    println!("Sub Result: {:016b}, Borrow: {}", res[0], res[1]); // Expected output: 0000000000000001, Borrow: 0

    let res = l16_logic::lor(a, b);
    println!("OR Result: {:016b}", res); // Expected output: 0000000000000011

    let res = l16_logic::land(a, b);
    println!("AND Result: {:016b}", res); // Expected output: 0000000000000010

    let res = l16_logic::lxor(a, b);
    println!("XOR Result: {:016b}", res); // Expected output: 0000000000000001

    let res = l16_logic::lnot(a);
    println!("NOT Result: {:016b}", res); // Expected output: 1111111111111100

    let res = l16_shift::bsl(a);
    println!("BSL Result: {:016b}", res); // Expected output: 0000000000000110

    let res = l16_shift::bsr(a);
    println!("BSR Result: {:016b}", res); // Expected output: 0000000000000001

    let flags = l16_compare::cmp(a, b); // Expected output: Equal: 0, Less Than: 0, Greater Than: 1
    println!("Compare Flags - Equal: {}, Less Than: {}, Greater Than: {}", flags[0], flags[1], flags[2]);
}
