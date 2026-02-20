#![allow(dead_code)]
#![allow(unused_variables)]

mod hash;
mod account;
mod keypair;
mod operation;
mod transaction;
mod block;
mod blockchain;


fn main() {
    let my_keys = keypair::KeyPair::new();
    
    println!("Public Key: {:?}", my_keys.public_key);
    
}
