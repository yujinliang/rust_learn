//reference: http://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/?utm_source=reddit&utm_medium=social&utm_campaign=rust-closures-in-ffi
use std::os::raw::{c_int};
mod closure;
use closure::add_two_numbers;

fn main() {
    let mut got = 0; 
    let closure = |result: c_int| {
        got = result; 
        println!("cb: {}", result);
    };
    add_two_numbers(1, 2, closure);
    println!("got: {}", got);
}

