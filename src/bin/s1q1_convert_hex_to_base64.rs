use base64::prelude::*;

// Converts a hex string into a base64 string following [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648).
pub fn hex_to_base64(hex: String) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    // Convert pairs of hex values to U8. Ex. 0xAF => 0b10101111.
    for i in 0..hex.len() / 2 {
        let b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match b {
            Ok(b) => bytes.push(b),
            Err(e) => panic!("unable to handle converting {} into Base64: {}", hex, e),
        }
    }
    BASE64_STANDARD.encode(bytes)
}
fn main() {
    let input = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let output = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    assert_eq!(hex_to_base64(input), output)
}
