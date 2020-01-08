extern crate sha3;

use sha3::{Digest, Keccak256};

fn main() {
    // create a Keccak256 object
    let mut hasher = Keccak256::new();

    // write input message
    hasher.input(b"abc");

    // read hash digest
    let result = hasher.result();

    println!("result: {:x}", result);
}
