//https://doc.rust-lang.org/nomicon/borrow-splitting.html
//https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let mut x = Foo {a: 0, b: 0, c: 0};
    let xx =&mut x;
    let a = &mut x.a;
    let b = &mut x.b;
    let c = &x.c;
    *b += 1;
    let c2 = &x.c;
    *a += 10;
    println!("{} {} {} {}", a, b, c, c2);
    //println!("whole struct: {:?}", xx);
    //Rust 编译器很聪明，允许你分别持有指向结构体子成员的可变引用，因为Rust编译器知道他们彼此没有交集，是安全的。
    //但是`引用xx`与后续声明的其他引用有交集，很明显struct整体当然包含其子成员！Rust铁律：允许多只读，排他写！所以不安全！
    //但是Rust编译器很聪明，只要`引用xx`无处使用就是安全的！所以注释掉println!就OK, 否则报错：` cannot borrow `x.a` as mutable more than once at a time`

}
