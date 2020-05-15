//RUSTFLAGS='-L .' cargo run
//-L 用于告诉rustc 库位置。

use std::os::raw::c_int; //(1) 必须使用rust and c都认识的数据类型。

//(2) 这个rust属性用于按名指定链接库，默认链接动态库，除非kind设定static指定链接静态库。
#[link(name="test")]
//#[link(name = "test", kind = "static")]

//(3) 申明外部函数遵守C语言函数调用规约。
extern "C"  {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    //(4) Rust 规定，只允许在unsafe块中调用FFI extern fn.
    let r = unsafe{add(2, 3)};
    println!("{}", r);
}
