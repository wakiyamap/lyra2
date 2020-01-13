use crate::lyra2;
use blake_hash::Blake256;
use digest::generic_array::typenum::U32;
use groestl::Groestl256;
use sha3::Keccak256;
use skein_hash::Digest;

pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let mut blake256 = Blake256::new();
    blake256.input(input);
    let result_blake = blake256.result();

    let mut keccak256 = Keccak256::new();
    keccak256.input(result_blake);
    let result_keccak256_1 = keccak256.result().to_vec();

    let result_keccak256_2 = result_keccak256_1.clone();

    let result_lyra2 = lyra2::lyra2(32, result_keccak256_1, result_keccak256_2, 1, 8, 8);

    let result_skein = skein_hash::Skein512::<U32>::digest(&result_lyra2);

    let mut groestl = Groestl256::new();
    groestl.input(result_skein);
    let result_groestl256 = groestl.result();

    result_groestl256.to_vec()
}

#[test]
fn lyra2re_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let lyra2re_result1 = sum(base1);
    assert_eq!(
        "07d3fe93103f6ad4284ad389d4b0a80334c94f5ffd0a537dfc935b3625552317",
        lyra2re_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base2 = "脇山珠美ちゃん可愛い！".as_bytes().to_vec();
    let lyra2re_result2 = sum(base2);
    assert_eq!(
        "dfcbbb6c85f5c8215b340caf8cac46b605c85b6c05403eaddc9dfc66750929e1",
        lyra2re_result2
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "😀😁😂".as_bytes().to_vec();
    let lyra2re_result3 = sum(base3);
    assert_eq!(
        "c3afc6db9914ab54f95ddd3daeee4f43f960d431a845de7dcae18544b14e8b8b",
        lyra2re_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
