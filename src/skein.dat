extern crate skein_hash;
extern crate digest;

use digest::generic_array::typenum::U32;
use skein_hash::Digest;

fn main() {
    // create a skein256 object
    //let mut hasher = skein_hash::Skein256::<U32>::new();

    // write input message
    //hasher.input(b"abc");

    // read hash digest
    //let result = hasher.result();
    println!("{:x}", skein_hash::Skein512::<U32>::digest(b"1833a9fa7cf4086bd5fda73da32e5a1d"));

    //println!("result: {:x}", result);
}
