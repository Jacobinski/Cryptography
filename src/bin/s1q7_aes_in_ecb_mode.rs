use base64::prelude::*;
use openssl::symm::{decrypt, Cipher};
use std::fs::read_to_string;

fn read_file(path: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let mut bytes = BASE64_STANDARD.decode(line).unwrap();
        out.append(&mut bytes);
    }
    out
}

fn main() {
    let cipher = Cipher::aes_128_ecb();
    let key = "YELLOW SUBMARINE".as_bytes();
    let encrypted = read_file("./src/bin/s1q7_input.txt");

    let decrypted = decrypt(cipher, key, None, &encrypted).unwrap();
    println!("{}", String::from_utf8(decrypted).unwrap());
}
