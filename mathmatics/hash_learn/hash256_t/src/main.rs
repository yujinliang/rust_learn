//https://github.com/RustCrypto
//https://docs.rs/sha2/0.8.1/sha2/
//https://github.com/RustCrypto/hashes/tree/master/sha2
use sha2::{Sha256, Sha512, Digest};

fn main() {
    
// create a Sha256 object
let mut hasher = Sha256::new();
// write input message
hasher.input(b"hello world");
// read hash digest and consume hasher
let result = hasher.result();
println!("Result: {:x}", result);

// same for Sha512
let mut hasher = Sha512::new();
hasher.input(b"hello world");
let result = hasher.result();
println!("Result: {:x}", result);

}
