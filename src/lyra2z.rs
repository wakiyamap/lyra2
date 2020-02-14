//! # lyra2z
//!
//! `lyra2z` crate has necessary formulas to calculate `lyra2z`.
use crate::lyra2;
use blake_hash::Blake256;
use skein_hash::Digest;

/// Returns the calculation result of lyra2z.
/// # Examples
///
/// ```
/// let base1 = "abc".as_bytes().to_vec();
/// let lyra2z_result1 = lyra2::lyra2z::sum(base1);
/// assert_eq!(
///     "cf9d13829886efd875cb0d01e44a80288d478346dd721fac0e6e04fe5774879c",
///     lyra2z_result1
///         .iter()
///         .map(|n| format!("{:02x}", n))
///         .collect::<String>()
/// );
/// ```
pub fn sum(input: Vec<u8>) -> Vec<u8> {
    let mut blake256 = Blake256::new();
    blake256.input(input);
    let result_blake_1 = blake256.result().to_vec();

    let result_blake_2 = result_blake_1.clone();

    lyra2::lyra2(32, result_blake_1, result_blake_2, 8, 8, 8)
}

#[test]
fn lyra2z_hash_cal() {
    let base1 = "abc".as_bytes().to_vec();
    let lyra2z_result1 = sum(base1);
    assert_eq!(
        "cf9d13829886efd875cb0d01e44a80288d478346dd721fac0e6e04fe5774879c",
        lyra2z_result1
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base2 = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let lyra2z_result2 = sum(base2);
    assert_eq!(
        "576aa33e47af09af117373008cfd98b452d605e1eab6f6fe173235a3b454e059",
        lyra2z_result2
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );

    let base3 = "😀😁😂".as_bytes().to_vec();
    let lyra2z_result3 = sum(base3);
    assert_eq!(
        "a226bea4d989b5eaf482c493921aff2608729d4cd9a1d6dda668b45b2fe02c8e",
        lyra2z_result3
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
