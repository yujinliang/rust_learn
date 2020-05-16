//注意：此处没有使用#[link]属性指定需要链接的C库， 因为我们在build.rs构建脚本中已经设定好了，
//rust cargo 知道该去链接那个C库。
extern "C" { fn hello(); }

fn main() {
    unsafe { hello(); }
}