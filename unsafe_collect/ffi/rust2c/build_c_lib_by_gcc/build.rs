// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

//下面直接调用gcc生成C库，并未考虑跨平台问题，切切！
    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    //上面的代码很直观，就是编译C 代码，构建静态库的命令行， 生成的C库存放到"OUT_DIR"环境变量指定的目录。
    //其实您完全可以举一反三， 通过编写build.rs构建脚本，可以调用诸如gcc, ar, make,cmake等C/C++构建工具为Rust工程提前生成C库。
    //我想您能想到， build.rs肯定是在开始构建编译Rust工程之前执行的！用于预处理。
    
    //下面很关键，配置cargo的官方指定方法之一 ！
    println!("cargo:rustc-link-search=native={}", out_dir); //配置C库的搜索路径，相当于rustc -L
    println!("cargo:rustc-link-lib=static=hello"); //配置需要链接的C库名, 相当于rustc -l
    println!("cargo:rerun-if-changed=src/hello.c"); //告诉cargo工具，只有当“src/hello.c”这个文件发生变化时，才重新执行build.rs脚本。
}