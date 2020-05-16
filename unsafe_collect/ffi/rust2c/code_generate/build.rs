// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    //"OUT_DIR" 告诉cargo 此build脚本的output应该存放到什么位置。
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World!\"
        }
        "
    ).unwrap();

    //注意哟：这不是普通的print呀， 这是配置cargo的一种官方方法。
    //“rerun-if-changed”是cargo 指令，下面代码的意思是：只有当build.rs脚本文件发生变化时，才重新执行build.rs，
    //否则默认只要package里的文件发生变化，就re-run build.rs。
    println!("cargo:rerun-if-changed=build.rs");
}