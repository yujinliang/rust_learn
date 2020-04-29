# `Rust 不允许C++方式的函数重载overloading`

C++方式的函数重载，即同一个函数名以及多个不同的形参类型和个数（不包括返回值类型）， 以Ad-hoc(临时，随时，不用事先深思熟虑)的方式来实现函数的重载！功能非常强大， 同时也是惹祸根源之一！

Rust 只允许通过预先定义和实现Trait的方式来近似模拟C++ ad-hoc 函数重载！比如Rust允许部分运算符重载，比如:std::ops::Add Trait , 只要为你的自定义类型实现了这个Add Trait 那么你的自定义类型就可以执行加法运算，如： a+b 。

```rust
use std::ops::Add;

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
struct Complex {
    real : i32,
    imag: i32,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}
fn main() {
    let c1 = Complex{ real: 3, imag:7};
    let c2 = Complex{ real: 4, imag:6};
    println!("{:?}", c1 +c2); //对+运算符的重载。
}

```



- `通过Rust Trait来模拟C++ ad-hoc函数重载`

```rust
#[derive(Debug)]
enum IntOrFloat {
    Int(i32),
    Float(f32),
}

trait IntOrFloatTrait {
    fn to_int_or_float(&self) -> IntOrFloat;
}

impl IntOrFloatTrait for i32 {
    fn to_int_or_float(&self) -> IntOrFloat {
        IntOrFloat::Int(*self)
    }
}

impl IntOrFloatTrait for f32 {
    fn to_int_or_float(&self) -> IntOrFloat {
        IntOrFloat::Float(*self)
    }
}

fn attempt_4<T: IntOrFloatTrait>(x: T) {
    let v = x.to_int_or_float();
    println!("{:?}", v);
}

fn main() {
    let i: i32 = 1;
    let f: f32 = 3.0;

    //从表面上看，实现了同一个函数名和不同的参数类型。
    //从本质来说，它只是通过trait来实现的自动类型转换而已，只是语法糖。
    //Rust官方也是通过trait来实现模拟函数重载的，包括运算符重载都是采用统一模式， 即定义和实现相应trait。
    attempt_4(i);
    attempt_4(f);
}
```

> Rust 本质上禁止C++ ad-hoc 函数重载，因为坑太深！但是又通过trait来实现了一定的灵活性！如果再结合上泛型，那就会强大无比，而且更加安全可靠， 可谓严肃活泼！
>
> 我认为：所有权、生命周期、借用和Trait是Rust的灵魂特性。对于`Rust Trait`即可以帮你填平类型的差异，又可以帮你差异化定制，慢慢体会吧。
>
> Rust 官方也是通过这种模式来模拟C++ ad hoc函数重载的！标准库中很容易找到类似模式代码。



```rust
#[derive(Debug)]
struct Foo {
    value:u64
  }
  
  trait HasUIntValue {
    fn as_u64(self) -> u64;
  }
  
  impl Foo {
    fn add<T:HasUIntValue>(&mut self, value:T) {
      self.value += value.as_u64();
    }
  }
  
  impl HasUIntValue for i64 {
    fn as_u64(self) -> u64 {
      return self as u64;
    }
  }
  
  impl HasUIntValue for f64 {
    fn as_u64(self) -> u64 {
      return self as u64;
    }
  }
  
  fn test_add_with_int()
  {
    let mut x = Foo { value: 10 };
    x.add(10i64);
    assert!(x.value == 20);
    println!("{:?}", x);
  }
  
    fn test_add_with_float()
  {
    let mut x = Foo { value: 10 };
    x.add(10.0f64);
    assert!(x.value == 20);
    println!("{:?}", x);
  }


fn main() {
    test_add_with_int();
    test_add_with_float();
}

```

> 万变不离其宗，只有明确实现了相应的`Trait`才可能具有相应的能力，才允许调用相应的函数方法， 从而有效避免了C++ ad-hoc函数重载的不可控和不明确问题。



- Reference

> `https://stackoverflow.com/questions/24857831/is-there-any-downside-to-overloading-functions-in-rust-using-a-trait-generic-f`
>
> `https://stackoverflow.com/questions/24936872/how-do-i-use-parameter-overloading-or-optional-parameters-in-rust`
>
> `https://stackoverflow.com/questions/25265527/how-can-i-approximate-method-overloading`
>
> 《深入浅出Rust》 范长春著， 机械工业出版社



- Author

> 学习随笔，如有谬误，望请海涵雅正，谢谢。
>
> 作者：心尘了
>
> email: [285779289@qq.com](mailto:285779289@qq.com)







