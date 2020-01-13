lyra2
====
## Description
Pure rust library in Lyra2, Lyra2RE, Lyra2REv2, Lyra2REv3.

The following golang libraries have been rewritten in Rust.
https://github.com/bitgoin/lyra2rev2

## Example
```
extern crate lyra2;

fn main() {
    let base1 = "abc".as_bytes().to_vec();
    let base2 = base1.clone();
    let lyra2_result1 = lyra2::lyra2::lyra2(32, base1, base2, 1, 4, 4);
    println!("result: {:?}", lyra2_result1); 
    //result: [143, 99, 117, 139, 209, 120, 240, 20, 234, 63, 212, 223, 9, 255, 10, 97, 100, 109, 197, 116, 160, 182, 188, 242, 137, 14, 197, 41, 166, 167, 54, 12]
}
```
```
extern crate lyra2;

fn main() {
    let base = "脇山珠美ちゃん可愛い！".as_bytes().to_vec();
    let lyra2rev2_result = lyra2::lyra2rev2::sum(base);
    assert_eq!(
        "d355b36923e5db0a035cca09c3ca6aab1081a4fc95ddc8210e11552aa64440b6",
        lyra2rev2_result
            .iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>()
    );
}
```
```
extern crate lyra2;

fn main() {
    let base = "testtesttest".as_bytes().to_vec();
    let lyra2rev3_result = lyra2::lyra2rev3::sum(base);
    assert_eq!(
        "94148BC4357A4BB36CA54A8CCA9584BEBAF785E43DE4DBB2C3F630F3156572CE",
        lyra2rev3_result
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<String>()
    );
}
```
## Installation
In order to use this crate, you have to add it under ``[dependencies]`` to your ``Cargo.toml``
```
[dependencies]
lyra2 = "0.1.0"
```

## Licence
MIT
