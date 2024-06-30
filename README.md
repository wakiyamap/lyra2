lyra2
====

[![Rust](https://github.com/wakiyamap/lyra2/workflows/Rust/badge.svg)](https://github.com/wakiyamap/lyra2/actions)

## Description
Pure rust library in Lyra2, Lyra2RE, Lyra2REv2, Lyra2REv3.

The following golang libraries have been rewritten in Rust.
https://github.com/bitgoin/lyra2rev2

## Minimal rust version
1.71+

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
    let base = "脇山珠美ちゃんかわいい！".as_bytes().to_vec();
    let lyra2rev2_result = lyra2::lyra2rev2::sum(base);
    assert_eq!(
        "bdaaa569c4f4918da66b02f2d0a2093a51e3d1735ee6023e9a93185c7bff40bc",
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
    let base3 = parse_hex("700000005d385ba114d079971b29a9418fd0549e7d68a95c7f168621a314201000000000578586d149fd07b22f3a8a347c516de7052f034d2b76ff68e0d6ecff9b77a45489e3fd511732011df0731000");
    let lyra2rev3_result1 = lyra2::lyra2rev3::sum(base3);
    println!("result: {:?}", lyra2rev3_result1.iter()
            .map(|n| format!("{:02x}", n))
            .collect::<String>());
            //result: "5d7b298258e78881c7831ba1e46751b089efdf1fdb9eb01edd03b8d7ed39eafb"
}

// from code https://tutorialmore.com/questions-23721.htm
fn parse_hex(hex_asm: &str) -> Vec<u8> {
    let hex_chars: Vec<char> = hex_asm.as_bytes().iter().filter_map(|b| {
        let ch = char::from(*b);
        if ('0' <= ch && ch <= '9') || ('a' <= ch && ch <= 'f') || ('A' <= ch && ch <= 'F') {
            Some(ch)
        } else {
            None
        }
    }).collect();
    let mut index = 0usize;
    let (odd_chars, even_chars): (Vec<char>, Vec<char>) = hex_chars.into_iter().partition(|_| { 
        index = index + 1;
        index % 2 == 1
    });
    odd_chars.into_iter().zip(even_chars.into_iter()).map(|(c0, c1)| {
        fn hexchar2int(ch: char) -> u8 {
            if '0' <= ch && ch <= '9' {
                ch as u8 - '0' as u8
            } else {
                0xa + 
                if 'a' <= ch && ch <= 'f' {
                    ch as u8 - 'a' as u8
                } else if 'A' <= ch && ch <= 'F' {
                    ch as u8 - 'A' as u8
                } else {
                    unreachable!()
                }
            }
        }
        hexchar2int(c0) * 0x10 + hexchar2int(c1)            
    }).collect::<Vec<u8>>()
}
```
## Installation
In order to use this crate, you have to add it under ``[dependencies]`` to your ``Cargo.toml``
```
[dependencies]
lyra2 = "0.2.7"
```

## License

All crates licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
