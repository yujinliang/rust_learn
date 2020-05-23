fn main() {
    cc::Build::new()
    .file("src/closure.c")
    .compile("closure");
    println!("cargo:rerun-if-changed=src/closure.c");
}