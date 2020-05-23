fn main() {
    cc::Build::new()
    .file("src/simple.c")
    .compile("simple");
    println!("cargo:rerun-if-changed=src/simple.c");

}