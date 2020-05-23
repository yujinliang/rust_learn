// src/main.rs
//reference: https://github.com/Michael-F-Bryan/rust-closures-and-ffi
//reference:http://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/?utm_source=reddit&utm_medium=social&utm_campaign=rust-closures-in-ffi

mod simple;
use simple::{ simple_add_two_numbers, two_numbers_added_cb};

fn main() {
    let a = 1;
    let b = 2;

    println!("Adding {} and {}", a, b);

    unsafe {
        simple_add_two_numbers(1, 2, two_numbers_added_cb);
    }
}
