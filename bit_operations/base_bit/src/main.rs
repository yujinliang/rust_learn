fn main() {
    let mut byte: u8 = 0b0000_0000;
    println!("0b{:08b}", byte);

    byte |= 0b0000_1000; // Set a bit
    println!("0b{:08b}", byte);

    byte &= 0b1111_0111; // Unset a bit
    println!("0b{:08b}", byte);

    byte ^= 0b0000_1000; // Toggle a bit
    println!("0b{:08b}", byte);

    byte = !byte; // Flip all bits
    println!("0b{:08b}", byte);

    byte <<= 1; // shift left one bit
    println!("0b{:08b}", byte);

    byte >>= 1; // shift right one bit
    println!("0b{:08b}", byte);

    //https://doc.rust-lang.org/std/primitive.u8.html
    let mut rbyte: u8 = 0b1000_0000;
    rbyte = rbyte.rotate_left(1); // rotate left one bit
    println!("0b{:08b}", byte);
    //https://doc.rust-lang.org/std/#primitives
    rbyte = rbyte.rotate_right(1); // rotate right one bit
    println!("0b{:08b}", rbyte);

    bit_twiddling(0, 3);
    bit_twiddling(8, 3);

    //test bitwise operation macros
    assert_eq!(eq1!(0b0000_1111, 0), true);
    assert_eq!(eq0!(0b0000_1111, 4), true);
    assert_eq!(set!(0b0000_1111, 0), 0x0f);
    assert_eq!(clr!(0b0000_1111, 0), 0x0e);
}

fn bit_twiddling(original: u8, bit: u8) {
    let mask = 1 << bit;

    println!(
        "Original: {:b}, Set: {:b}, Cleared: {:b}, Toggled: {:b}",
        original,
        original |  mask,
        original & !mask,
        original ^  mask
    );
}

//define rust macro for bitwise operations
#[macro_export]
macro_rules! eq1 {
    ($n:expr, $b:expr) => {
        $n & (1 << $b) != 0
    };
}

#[macro_export]
macro_rules! eq0 {
    ($n:expr, $b:expr) => {
        $n & (1 << $b) == 0
    };
}

#[macro_export]
macro_rules! set {
    ($n:expr, $b:expr) => {
        $n | (1 << $b)
    };
}

#[macro_export]
macro_rules! clr {
    ($n:expr, $b:expr) => {
        $n & !(1 << $b)
    };
}