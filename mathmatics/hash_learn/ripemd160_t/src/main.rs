//ripemd160
//https://docs.rs/ripemd160/0.8.0/ripemd160/
//https://github.com/RustCrypto/hashes
use ripemd160::{Ripemd160, Digest};

fn main() {
    // create a RIPEMD-160 hasher instance
let mut hasher = Ripemd160::new();
// process input message
hasher.input(b"Hello world!");
// acquire hash digest in the form of GenericArray,
// which in this case is equivalent to [u8; 20]
let result = hasher.result();
println!("{:x}", result);
}
