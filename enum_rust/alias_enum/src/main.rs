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

