//关键点：此行宏代码将build.rs生成的代码文件包含进来加入编译。
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
}