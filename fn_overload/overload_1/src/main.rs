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

    attempt_4(i);
    attempt_4(f);
}