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

    //特别提醒：rust为每一个数字类型都实现了大量方法，其中包括位操作方法！！！具体请参看下方链接！！！
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



- `[Crate bitflags] map struct to a bit flag set`

```rust
use std::fmt;

#[macro_use]
extern crate bitflags;

bitflags! {
    pub struct Flags: u32 {//模块可见性: private默认本模块可见， pub模块外也可见。
        const A = 0b00000001;  
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;  // The `bits` field can be accessed from within the
                        // same module where the `bitflags!` macro was invoked.
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi!")
    }
}

fn main() {
    //example1
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
    assert!(e1.contains(Flags::A));
    //example2
    let mut flags = Flags::A | Flags::B;
    flags.clear();
    assert!(flags.is_empty());
    assert_eq!(format!("{}", flags), "hi!");
    assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
    assert_eq!(format!("{:?}", Flags::B), "B");
}
```

> `Crate [bitflags](https://docs.rs/bitflags/1.2.1/bitflags/)` 此Rust Crate可以将一个`struct`转化为一个`bit flags set`, 自动完成映设和转化， 此处代码例子出自它的文档， 若要深入了解可去详细阅读之。



- `[Crate enumflags2] map a enum to a bit flags set`

```rust
use enumflags2::BitFlags;

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
enum Test {
    A = 0b0001,
    B = 0b0010,
    C = 0b0100,
    D = 0b1000,
}

fn main() {

    let a_b = Test::A | Test::B; // BitFlags<Test>
    let a_c = Test::A | Test::C;
    let b_c_d = Test::C | Test::B | Test::D;
    
    // BitFlags<Test>(0b11, [A, B])
    println!("{:?}", a_b);
    
    // BitFlags<Test>(0b1, [A])
    println!("{:?}", a_b & a_c);
    
    // Iterate over the flags like a normal set!
    assert_eq!(a_b.iter().collect::<Vec<_>>(), &[Test::A, Test::B]);
    
    assert!(a_b.contains(Test::A));
    assert!(b_c_d.contains(Test::B | Test::C));
    assert!(!(b_c_d.contains(a_b)));
    
    assert!(a_b.intersects(a_c));
    assert!(!(a_b.intersects(Test::C | Test::D)));
}

```

> `https://docs.rs/enumflags2/0.6.4/enumflags2/` 此crate 将一个`rust enum`映设为一个`bit flags set!` 此处代码例子出自它的文档， 若要深入了解可去详细阅读之。
>
> 看来一定的抽象和封装是应对复杂和繁琐的有效手段！一定的抽象和封装使枯燥繁琐的位操作具有更好的可读性，更强的表达能力， 类型和命名是个好武器，将位映设关系固化自动化！从而降低使用难度，降低出错率。
>
> 但是人们没有就此止步，因为实际嵌入式编程中，我们直接与一个个`寄存器`交互， 从而控制硬件！所以`寄存器`只需暴露一个个`API`就好，而其内部的位操作和映设关系最好封装起来，外部无需了解！从而降低使用复杂度！实际编码中，我们只需要定义好一个个`寄存器`， 然后调用其`API`就好， 清晰明确，不易出错,介绍一个这样的`rust crate`  : `bounded_registers` ,  ` [bounded_registers](https://docs.rs/bounded-registers/0.1.1/bounded_registers/)`  , 其设计了形式化的格式来定义每一个`寄存器`， 然后定义了统一的方法操作`寄存器`的状态值，若要深入了解，请参阅其文档。



- 结束语

`我们经常说，计算机编程领域应对复杂繁琐的有效武器之一就是：“增加一层，将复杂和繁琐抽象出来，封装起来形成一层代理”`  上面的代码例子都比较直观，无需我多言，您若要深入研究，可参阅其原始文档和代码，本文只当抛砖引玉而已！



- 作者

> 学习随笔，如有谬误，望请海涵雅正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)



- Reference

> `https://opensource.com/article/20/1/c-vs-rust-abstractions`
>
> `https://docs.rs/bitflags/1.2.1/bitflags/`
> `https://docs.rs/bounded-registers/0.1.1/bounded_registers/`
> `https://stackoverflow.com/questions/43509560/how-to-implement-bitwise-operations-on-a-bitflags-enum`
> `https://docs.rs/enumflags2/0.6.4/enumflags2/`
>
> `https://stackoverflow.com/questions/40467995/how-do-you-set-clear-and-toggle-a-single-bit-in-rust`
>
> `https://doc.rust-lang.org/std/primitive.i8.html`
> `https://doc.rust-lang.org/std/primitive.u8.html`
> `https://doc.rust-lang.org/std/#primitives`
>
> `https://doc.rust-lang.org/book/appendix-02-operators.html`
> `https://github.com/rsdump/rit`
> `https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators`
> `https://rust-lang-nursery.github.io/rust-cookbook/data_structures/bitfield.html`





