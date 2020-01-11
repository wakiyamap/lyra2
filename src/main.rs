mod lyra2;
mod cubehash;
mod bmw;
extern crate blake_hash;
extern crate skein_hash;
extern crate sha3;
use bytes::Bytes;
use sha3::Keccak256;
use skein_hash::Digest;
use digest::generic_array::typenum::U32;

fn main() {

	let mut blake256 = blake_hash::Blake256::new();
	blake256.input(b"abc");
	let result_blake = blake256.result();

	let mut keccak256 = Keccak256::new();
	keccak256.input(result_blake);
	let result_keccak256 = keccak256.result();

	let result_cube1 = cubehash::sum(result_keccak256.to_vec());
	let result_cube2 = result_cube1.clone();

	let mut lyra2result: Vec<u8> = "00000000000000000000000000000000".as_bytes().to_vec();
	lyra2result = lyra2::sum(lyra2result, result_cube1, result_cube2, 1, 4, 4);

	let result_skein = skein_hash::Skein512::<U32>::digest(&lyra2result);

	let result_cube3 = cubehash::sum(result_skein.to_vec());

	let result_bmw = bmw::sum(result_cube3);
	let result = Bytes::from(result_bmw);
	println!("result: {:x}", result);
	println!("result: {}", result.iter().map(|n| format!("{:02x}", n)).collect::<String>());
}
