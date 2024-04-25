use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Bytes {
    data: Vec<u8>,
}

#[derive(Debug)]
struct InvalidInput;

impl Bytes {
    fn from_hex_string(hex: String) -> Result<Bytes, InvalidInput> {
        let mut data: Vec<u8> = Vec::new();
        // Convert pairs of hex values to U8. Ex. 0xAF => 0b10101111.
        for i in 0..hex.len() / 2 {
            let b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
            match b {
                Ok(b) => data.push(b),
                Err(_) => return Err(InvalidInput),
            }
        }
        if hex.len() % 2 != 0 {
            let b = u8::from_str_radix(&hex[hex.len() - 1..hex.len()], 16);
            match b {
                Ok(b) => data.push(b),
                Err(_) => return Err(InvalidInput),
            }
        }
        Ok(Bytes { data })
    }

    fn xor(&self, byte: u8) -> Bytes {
        let mut data: Vec<u8> = Vec::new();
        for d in self.data.iter() {
            data.push(d ^ byte);
        }
        Bytes { data }
    }

    #[allow(dead_code)]
    fn alphabet_frequency_score(&self) -> f64 {
        // Taken from https://pi.math.cornell.edu/~mec/2003-2004/cryptography/subs/hints.html
        let character_frequencies = HashMap::from([
            (b'e', 21912.0 / 40000.0),
            (b't', 16587.0 / 40000.0),
            (b'a', 14810.0 / 40000.0),
            (b'o', 14003.0 / 40000.0),
            (b'i', 13318.0 / 40000.0),
            (b'n', 12666.0 / 40000.0),
            (b's', 11450.0 / 40000.0),
            (b'r', 10977.0 / 40000.0),
            (b'h', 10795.0 / 40000.0),
            (b'd', 7874.0 / 40000.0),
            (b'l', 7253.0 / 40000.0),
            (b'u', 5246.0 / 40000.0),
            (b'c', 4943.0 / 40000.0),
            (b'm', 4761.0 / 40000.0),
            (b'f', 4200.0 / 40000.0),
            (b'y', 3853.0 / 40000.0),
            (b'w', 3819.0 / 40000.0),
            (b'g', 3693.0 / 40000.0),
            (b'p', 3316.0 / 40000.0),
            (b'b', 2715.0 / 40000.0),
            (b'v', 2019.0 / 40000.0),
            (b'k', 1257.0 / 40000.0),
            (b'x', 315.0 / 40000.0),
            (b'q', 205.0 / 40000.0),
            (b'j', 188.0 / 40000.0),
            (b'z', 128.0 / 40000.0),
            (b' ', 0.0),
            (b'\'', 0.0),
            (b'.', 0.0),
            (b',', 0.0),
        ]);

        self.data.iter().copied().fold(0.0, |mut score, byte| {
            let cf = match character_frequencies.get(&byte) {
                Some(v) => v,
                None => &0.0,
            };
            score += cf;
            score
        })
    }

    fn to_ascii(&self) -> String {
        let mut s = Vec::new();
        for b in self.data.iter().cloned() {
            s.push((b as char).to_string())
        }
        s.join("")
    }
}

fn main() {
    let file = File::open("./src/bin/s1q4_input.txt").expect("exists");
    let br = io::BufReader::new(file);

    let mut best = f64::MIN;
    let mut best_input = String::new();
    let mut best_string = String::new();
    for line in br.lines() {
        let input = line.expect("exists");
        let input_bytes = Bytes::from_hex_string(input.clone()).expect("success");

        for b in 0b0000_0000..=0b1111_1111 {
            let new_bytes = input_bytes.xor(b);
            let score = new_bytes.alphabet_frequency_score();
            let ascii = new_bytes.to_ascii();
            if score > best {
                best = score;
                best_input = input.clone();
                best_string = ascii;
            }
        }
    }

    println!(
        "Found encrypted text {} from input {}",
        best_string, best_input
    );
    assert_eq!(best_string, String::from("Now that the party is jumping\n"))
}
