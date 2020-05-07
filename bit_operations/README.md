# Rust 位操作(Bitwise Operations )

- 基础篇

```rust
fn main() {
    
    //(1)最原始直接基础的位操作方法。
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

    //特别提醒：rust为每一位 数字类型都实现了大量方法，其中包括位操作方法！！！具体请参看下方链接！！！
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

//(2)定义为一个rust函数
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
//(3)定义为rust 宏
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
```

> 从C/C++一脉相传，位操作基本就是上面的样子！与或非，左移位、右移位等的组合。直说缺点吧，不好记忆，容易混淆！！！实际的嵌入式编程时，可能需要应对非常多的寄存器和每个寄存器bits的的映设关系！一旦出错不好排查！所以大家就想如果可以将位操作和rust的类型系统绑定起来，抽象封装成一个个类型和有意义的名字， 将映设关系固化下来，并且自动完成转化！从而增强语义和表达力，这样会很好用且容易排查错误！所以随后介绍的一些crates就是这方面的努力成果。











