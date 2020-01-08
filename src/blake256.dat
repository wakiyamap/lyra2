extern crate blake_hash;

use blake_hash::{Digest, Blake256};


fn main() {
    let mut hasher = Blake256::new();

    hasher.input(b"abc");

    let result = hasher.result();

    println!("result: {:x}", result);
}
