#![allow(dead_code)]

//https://doc.rust-lang.org/nomicon/other-reprs.html
//#[repr(C)]
#[repr(u8)]
enum Number {
    Zero, //默认从0开始。
    One = 22,
    Two,
}

// enum with explicit discriminator
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as u8);
    println!("one is {}", Number::One as u8);
    println!("one is {}", Number::Two as u8);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let color = Color::Blue;
    if color == Color::Blue { //must implement PartialEq trait.
        println!("{:?}", color);
    }
}
