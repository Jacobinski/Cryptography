extern crate lazy_static;

use base64::prelude::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

lazy_static::lazy_static! {
    // FREQUENCIES gives the relative frequencies of English characters.
    // Input must be in the form of a lowercase ASCII character's byte representation.
    // Source: https://web.archive.org/web/20170918020907/http://www.data-compression.com/english.html
    static ref FREQUENCIES: HashMap<u8, f32> = HashMap::from([
        (b'a', 0.0651738),
        (b'b', 0.0124248),
        (b'c', 0.0217339),
        (b'd', 0.0349835),
        (b'e', 0.1041442),
        (b'f', 0.0197881),
        (b'g', 0.0158610),
        (b'h', 0.0492888),
        (b'i', 0.0558094),
        (b'j', 0.0009033),
        (b'k', 0.0050529),
        (b'l', 0.0331490),
        (b'm', 0.0202124),
        (b'n', 0.0564513),
        (b'o', 0.0596302),
        (b'p', 0.0137645),
        (b'q', 0.0008606),
        (b'r', 0.0497563),
        (b's', 0.0515760),
        (b't', 0.0729357),
        (b'u', 0.0225134),
        (b'v', 0.0082903),
        (b'w', 0.0171272),
        (b'x', 0.0013692),
        (b'y', 0.0145984),
        (b'z', 0.0007836),
        (b' ', 0.1918182),
    ]);
}

fn score(data: Vec<u8>) -> f32 {
    data.iter()
        .map(|x| match FREQUENCIES.get(x) {
            Some(v) => v,
            None => &0.0,
        })
        .sum()
}

fn xor(data: &Vec<u8>, byte: u8) -> Vec<u8> {
    data.iter().map(|x| x ^ byte).collect()
}

fn open_file(path: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let mut bytes = BASE64_STANDARD.decode(line).unwrap();
        out.append(&mut bytes);
    }
    out
}

fn hamming(a: &[u8], b: &[u8]) -> u32 {
    zip(a, b).fold(0, |mut acc, (x, y)| {
        acc += (x ^ y).count_ones();
        acc
    })
}

fn edit_distance_for_keysize(data: &[u8], keysize: usize) -> f32 {
    let mut results = Vec::new();
    for i in 0..(data.len() / keysize) - 2 {
        results.push(
            hamming(
                &data[i * keysize..(i + 1) * keysize],
                &data[(i + 1) * keysize..(i + 2) * keysize],
            ) as f32
                / keysize as f32,
        );
    }
    let sum: f32 = results.iter().sum();
    let avg = sum / results.len() as f32;
    avg
}

fn transpose_bytes(input: &Vec<u8>, size: usize) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    for _ in 0..size {
        output.push(Vec::new());
    }
    for (i, byte) in input.iter().cloned().enumerate() {
        let v = output.get_mut(i % size).unwrap();
        v.push(byte);
    }
    output
}

fn decrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decrypted = Vec::new();
    let mut k = 0;
    for i in input {
        decrypted.push(i ^ key[k]);
        k = (k + 1) % key.len();
    }
    decrypted
}

#[derive(Clone)]
struct Guess {
    size: usize,
    distance: f32,
}

/*
Theory behind this solution:
https://crypto.stackexchange.com/questions/8115/repeating-key-xor-and-hamming-distance
https://carterbancroft.com/breaking-repeating-key-xor-theory/
*/
fn main() {
    assert_eq!(
        hamming("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
        37
    );

    let encrypted = open_file("./src/bin/s1q6_input.txt");

    let mut edit_distances = Vec::new();
    for size in 2..41 {
        let distance = edit_distance_for_keysize(&encrypted, size);
        edit_distances.push(Guess { size, distance });
    }

    let mut best_size = 0;
    let mut min_distance = f32::MAX;
    for guess in edit_distances.iter().cloned() {
        if guess.distance < min_distance {
            min_distance = guess.distance;
            best_size = guess.size;
        }
    }

    // println!("Best size {} with score {}\n", best_size, min_distance);
    let mut solution = Vec::new();
    let transposed_bytes = transpose_bytes(&encrypted, best_size);
    for byte_vec in transposed_bytes.iter() {
        let mut max_score = f32::MIN;
        let mut best_key = 0;
        for key in 0b00000000..=0b11111111 {
            let decrypted = xor(byte_vec, key);
            let score = score(decrypted);
            if score > max_score {
                max_score = score;
                best_key = key;
            }
        }
        solution.push(best_key);
    }

    let solution_text = String::from_utf8(solution.clone()).unwrap();
    println!("Found Cipher:\n{}\n", solution_text);
    assert_eq!(solution_text, "Terminator X: Bring the noise");

    let decrypted = decrypt(&encrypted, &solution);
    println!("Decrypted Text:\n{}", String::from_utf8(decrypted).unwrap());
}
