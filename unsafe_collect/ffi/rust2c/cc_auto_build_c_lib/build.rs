fn main() {
    //the cc crate专门自动构建编译C/C++ code,
    //如：自动检测：系统平台， 硬件架构， 自动选择相应编译器，设定各种编译参数，
    //自动设定相关环境变量， 如：cargo相关环境变量， 自动将编译好的C库保存到“OUT_DIR”
    //所以cc可以自动帮你搞定诸如：交叉编译， 跨平台。
    //cargo build -vv 可以看到已经自动设定的各种构建参数。
    //详情请参考：`https://docs.rs/cc/1.0.53/cc/` 
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
    println!("cargo:rerun-if-changed=src/hello.c"); //告诉cargo 只有当src/hello.c发生变化时，才重新执行build.rs脚本。
}