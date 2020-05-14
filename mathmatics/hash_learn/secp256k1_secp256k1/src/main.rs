//secp256k1
//https://docs.rs/secp256k1/0.17.2/secp256k1/
extern crate secp256k1;
use self::secp256k1::{Secp256k1, Message, SecretKey, PublicKey};

fn main() {

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
    
    let sig = secp.sign(&message, &secret_key);
    assert!(secp.verify(&message, &sig, &public_key).is_ok());

}
