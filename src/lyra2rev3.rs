//! # lyra2rev3
//!
//! `lyra2rev3` crate has necessary formulas to calculate `lyra2rev3`. For vertcoin etc...
use crate::bmw;
use crate::cubehash;
use crate::lyra2mod;
use blake_hash::Digest;

/// Returns the calculation result of lyra2rev3.
/// # Examples
///
/// ```
/// let base1 = "abc".as_bytes().to_vec();
/// let lyra2rev3_result1 = lyra2::lyra2rev3::sum(base1);
/// assert_eq!(
///     "4e445087e28d294b3074e98fee860fb73d248a63150ea2d42bfeddd21c0b89ef",
///     lyra2rev3_result1
///         .iter()
///         .map(|n| format!("{:02x}", n))
///         .collect::<String>()
/// );
/// ```
pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let result_blake = blake_hash::Blake256::digest(&input).to_vec();

    let result_lyra2_mod_1 = lyra2mod::sum(result_blake);

    let result_cube = cubehash::sum(result_lyra2_mod_1);

    let result_lyra2_mod_2 = lyra2mod::sum(result_cube);

    bmw::sum(result_lyra2_mod_2)
}

#[test]
fn lyra2rev3_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let lyra2rev3_result1 = sum(base1);
    assert_eq!(
        "4e445087e28d294b3074e98fee860fb73d248a63150ea2d42bfeddd21c0b89ef",
        lyra2rev3_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base2 = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let lyra2rev3_result2 = sum(base2);
    assert_eq!(
        "322b5d87289dca02e50c882314ee6a43efd91579ca4717ff49715f4bb039fd78",
        lyra2rev3_result2
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "😀😁😂".as_bytes().to_vec();
    let lyra2rev3_result3 = sum(base3);
    assert_eq!(
        "22cb37cae128d61e3060bf45189ba9b206755728596fc8b054c5712b68c12bcb",
        lyra2rev3_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
