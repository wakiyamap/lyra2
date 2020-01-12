mod bmw;
mod cubehash;
use crate::lyra2;
use sha3::Keccak256;
use skein_hash::Digest;
use digest::generic_array::typenum::U32;

pub fn sum(input: Vec<u8>) -> Vec<u8>{

	let mut blake256 = blake_hash::Blake256::new();
	blake256.input(input);
	let result_blake = blake256.result();

	let mut keccak256 = Keccak256::new();
	keccak256.input(result_blake);
	let result_keccak256 = keccak256.result();

	let result_cube1 = cubehash::sum(result_keccak256.to_vec());
	let result_cube2 = result_cube1.clone();

	let lyra2result = lyra2::sum(32, result_cube1, result_cube2, 1, 4, 4);

	let result_skein = skein_hash::Skein512::<U32>::digest(&lyra2result);

	let result_cube3 = cubehash::sum(result_skein.to_vec());

	let result_bmw = bmw::sum(result_cube3);
	return result_bmw;
}

#[test]
fn lyra2rev2_hash_cal() {
	let base1 = "abc".as_bytes().to_vec();
	let lyra2rev2_result1 = sum(base1);
	assert_eq!("80ec5344227c5d0bfd63038f00c3fe5aecddd1a1122043b0a90b5fd67b1e8f32", lyra2rev2_result1.iter().map(|n| format!("{:02x}", n)).collect::<String>());

	let base2 = "脇山珠美ちゃん可愛い！".as_bytes().to_vec();
	let lyra2rev2_result2 = sum(base2);
	assert_eq!("d355b36923e5db0a035cca09c3ca6aab1081a4fc95ddc8210e11552aa64440b6", lyra2rev2_result2.iter().map(|n| format!("{:02x}", n)).collect::<String>());

	let base3 = "😀😁😂".as_bytes().to_vec();
	let lyra2rev2_result3 = sum(base3);
	assert_eq!("f88283ab841272a7c3803a380af32a0805448e9ea2e7c3809c4342afeeb040f1", lyra2rev2_result3.iter().map(|n| format!("{:02x}", n)).collect::<String>());
}
