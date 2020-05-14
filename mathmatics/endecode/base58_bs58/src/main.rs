//bs58 = "0.3.1"
//https://docs.rs/bs58/0.3.1/bs58/
//https://github.com/mycorrhiza/bs58-rs
use std::io::{self, Read,Write};


fn main() {
    //encode
    let mut input = Vec::<u8>::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let enc = bs58::encode(input).into_string();
    println!("{}", enc);

    //decode
    match bs58::decode(enc).into_vec() {
        Ok(vec) => io::stdout().write_all(&*vec).unwrap(),
        Err(err) => eprintln!("{}", err),
    };
}
