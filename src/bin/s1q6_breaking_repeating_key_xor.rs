use base64::prelude::*;
use std::fs::read_to_string;
use std::iter::zip;

fn hamming(a: &[u8], b: &[u8]) -> u32 {
    zip(a, b).fold(0, |mut acc, (x, y)| {
        acc += (x ^ y).count_ones();
        acc
    })
}

fn open_file(path: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for line in read_to_string(path).unwrap().lines() {
        let mut bytes = BASE64_STANDARD.decode(line).unwrap();
        out.append(&mut bytes);
    }
    out
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
        println!("size {} gives hamming {}", size, distance);
        edit_distances.push(Guess { size, distance });
    }

    let mut min_size = 0;
    let mut min_distance = f32::MAX;
    for guess in edit_distances.iter().cloned() {
        if guess.distance < min_distance {
            min_distance = guess.distance;
            min_size = guess.size;
        }
    }

    println!("Best size {} with score {}", min_size, min_distance);
}
