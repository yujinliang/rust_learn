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
    println!("{:?}", c1 +c2);
}
