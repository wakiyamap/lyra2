use crate::bmw;
use crate::cubehash;
use crate::lyra2;
use crate::lyra2mod;
use blake_hash::Blake256;
use digest::generic_array::typenum::U32;
use groestl::Groestl256;
use sha3::Keccak256;
use skein_hash::Digest;

pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let mut blake256 = Blake256::new();
    blake256.input(input);
    let result_blake_1 = blake256.result().to_vec();

    let result_blake_2 = result_blake_1.clone();

    let result_lyra2_mod_1 = lyra2mod::sum(32, result_blake_1, result_blake_2, 1, 4, 4);

    let result_cube1 = cubehash::sum(result_lyra2_mod_1);
    let result_cube2 = result_cube1.clone();

    let result_lyra2_mod_2 = lyra2::sum(32, result_cube1, result_cube2, 1, 4, 4);

    let result_bmw = bmw::sum(result_lyra2_mod_2);
    return result_bmw;
}

#[test]
fn lyra2rev3_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let lyra2rev3_result1 = sum(base1);
    assert_eq!(
        "1351c939361155f8350fcb3a21168cb75fcd9bb457ffb4a5ebedd5cbf22043e6",
        lyra2rev3_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base2 = "脇山珠美ちゃん可愛い！".as_bytes().to_vec();
    let lyra2rev3_result2 = sum(base2);
    assert_eq!(
        "88b3aae7bef4a9d43a28a1017e8ad2d1171e425724f9e70a968ec4ea671edbff",
        lyra2rev3_result2
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "😀😁😂".as_bytes().to_vec();
    let lyra2rev3_result3 = sum(base3);
    assert_eq!(
        "413456c97042128f66acf14fe1cad44aa5bbb8dceb10a137e84c9b472ec24591",
        lyra2rev3_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
