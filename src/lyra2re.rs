//! # lyra2re
//!
//! `lyra2re` crate has necessary formulas to calculate `lyra2re`.
use crate::lyra2;
use digest::generic_array::typenum::U32;
use blake_hash::Digest as blakeDigest;
use groestl;
use sha3;
use skein_hash::Digest;

/// Returns the calculation result of lyra2re.
/// # Examples
///
/// ```
/// let base1 = "abc".as_bytes().to_vec();
/// let lyra2re_result1 = lyra2::lyra2re::sum(base1);
/// assert_eq!(
///     "07d3fe93103f6ad4284ad389d4b0a80334c94f5ffd0a537dfc935b3625552317",
///     lyra2re_result1
///         .iter()
///         .map(|n| format!("{:02x}", n))
///         .collect::<String>()
/// );
/// ```
pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let result_blake = blake_hash::Blake256::digest(&input).to_vec();

    let result_keccak256_1 = sha3::Keccak256::digest(&result_blake).to_vec();

    let result_keccak256_2 = result_keccak256_1.clone();

    let result_lyra2 = lyra2::lyra2(32, result_keccak256_1, result_keccak256_2, 1, 8, 8);

    let result_skein = skein_hash::Skein512::<U32>::digest(&result_lyra2);

    let result_groestl256 = groestl::Groestl256::digest(&result_skein).to_vec();

    result_groestl256
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

    let base2 = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let lyra2re_result2 = sum(base2);
    assert_eq!(
        "8361fd2586e630cf7bc8209fb05d8d24fb549cee19b1c085c2cec79c61c6cc6e",
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
