# 							`Rust enum 杂记`

`rust enum`是典型的代数数据类型系统`ADT`, 可以传参，可以组合等，非常强大易用。



- `enum 定义`

```rust
enum WebEvent {
    // An `enum` may either be `unit-like`,
    //类似于：struct PageLoad ; // 一个empty struct。
    PageLoad,
    PageUnload,
    // like tuple structs,
    //类似于一个tuple struct。
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    //类似于c structure。
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event { //enum 模式匹配。
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```

> 代码例子摘自：`https://doc.rust-lang.org/rust-by-example/custom_types/enum.html` ， 例子中分别定义了3中`enum variants`.   非常类似于`empty struct、 tuple struct、 c like struct`等，  `rust enum`比`c enum`而言拥有更强的封装和抽象能力，而非简单的分类！`enum`对于rust而言极其重要，比如`enum Option<T>和enum Result<T, E>` , 所以我认为有必要啰嗦一下。



- 类型别名

```rust
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// 为上面的enum创建一个类型别名。
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        //注意Self 也是上面enum的类型别名。
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
  //我们可以通过它的别名来引用每个变体，而不是冗长和不方便的名字。
    let x = Operations::Add;
    let y = Operations::Subtract;
    println!("{:?}: {}",x,  x.run(3, 4));
    println!("{:?}: {}",y,  y.run(3, 4));
}

```



- `use 声明`

```rust
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    //手动指定需要公开出来的变体。
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    //将enum Work下的变体自动都公开出来。
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}
```



- `C Like enum`

```rust
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

```

> `repr（C）极其重要`。它的目的相当简单明确：做`C`所做的事情。字段的顺序、大小和对齐方式正是`C或C++`所期望的。你希望跨越`FFI`边界的任何类型都应该有`repr（C）`，因为C是编程界的通用语言。这对于更好地使用数据布局（如将值重新解释为不同类型）进行更精细的控制技巧也是必要的。
>
> 我们强烈建议您使用`rust bindgen和/或cbindgen`来管理您的`FFI`边界。Rust团队与这些项目密切合作，以确保它们工作可靠，并与当前和未来有关类型布局和`reprs`保证兼容。
>
> 由于`repr（C）`具有“用于`FFI`”和“用于布局控制”的双重用途，因此它可以应用于那些如果跨越`FFI`边界将是无意义的或有问题的类型。



- `enum variants模式匹配`

> if let 、 while let 、 match



- `rust enum替代bool`

> `bool`只可以表达二态， 而`enum`不仅可以表达二态而且可以表达多态，而且令代码更具可读性，并且更易理解！`rust编译器`对于`enum`的检查也更加严格，比如未覆盖所有`enum variants` , 详情请参考原文：`http://blakesmith.me/2019/05/07/rust-patterns-enums-instead-of-booleans.html` ， 最重要的是：`rust enum`不仅可以表达多种状态（事件/类型等），而且每个状态可以封装携带数据，再配合上`泛型、impl、trait` 等rust语言特性！ 所以使得`rust enum`的抽象表达能力更强！



- Author

> 学习随笔，如有谬误，尽请指正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)
>
> 微信：13718438106



- Reference

> `https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html`
>
> `http://www.rust-compare.com/site/enums.html`
>
> `https://doc.rust-lang.org/rust-by-example/custom_types/enum.html`
>
> `http://blakesmith.me/2019/05/07/rust-patterns-enums-instead-of-booleans.html`
>
> `https://doc.rust-lang.org/nomicon/other-reprs.html`
>
> `https://doc.rust-lang.org/reference/items/enumerations.html`
>
> `《深入浅出Rust》 范长春著 机械工业`