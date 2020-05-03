#![allow(dead_code)]

//https://doc.rust-lang.org/nomicon/other-reprs.html
#[repr(C)]
enum Number {
    Zero, //默认从0开始。
    One = 22,
    Two,
}

// enum with explicit discriminator
#[repr(C)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("one is {}", Number::Two as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}