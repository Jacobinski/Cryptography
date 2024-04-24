use std::collections::HashMap;

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
            (b'E', 21912.0 / 40000.0),
            (b'T', 16587.0 / 40000.0),
            (b'A', 14810.0 / 40000.0),
            (b'O', 14003.0 / 40000.0),
            (b'I', 13318.0 / 40000.0),
            (b'N', 12666.0 / 40000.0),
            (b'S', 11450.0 / 40000.0),
            (b'R', 10977.0 / 40000.0),
            (b'H', 10795.0 / 40000.0),
            (b'D', 7874.0 / 40000.0),
            (b'L', 7253.0 / 40000.0),
            (b'U', 5246.0 / 40000.0),
            (b'C', 4943.0 / 40000.0),
            (b'M', 4761.0 / 40000.0),
            (b'F', 4200.0 / 40000.0),
            (b'Y', 3853.0 / 40000.0),
            (b'W', 3819.0 / 40000.0),
            (b'G', 3693.0 / 40000.0),
            (b'P', 3316.0 / 40000.0),
            (b'B', 2715.0 / 40000.0),
            (b'V', 2019.0 / 40000.0),
            (b'K', 1257.0 / 40000.0),
            (b'X', 315.0 / 40000.0),
            (b'Q', 205.0 / 40000.0),
            (b'J', 188.0 / 40000.0),
            (b'Z', 128.0 / 40000.0),
        ]);
        let count = self.data.len() as f64;
        let self_frequencies = self
            .data
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val.to_ascii_uppercase())
                    .and_modify(|v| *v += 1.0 / count)
                    .or_insert(0.0);
                map
            });

        let mut score = 0.0;
        for b in 0b0000_0000..=0b1111_1111 {
            let cf = match character_frequencies.get(&b) {
                Some(v) => v,
                None => &0.0,
            };
            let sf = match self_frequencies.get(&b) {
                Some(v) => v,
                None => &0.0,
            };
            score += f64::abs(cf - sf);
        }
        score
    }

    fn valid_ascii_score(&self) -> f64 {
        self.data.iter().copied().fold(0.0, |mut sum, byte| {
            if (b'A' <= byte && byte <= b'Z') || (b'a' <= byte && byte <= b'z') {
                sum += 1.0;
            }
            sum
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
    let input =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let input_bytes = Bytes::from_hex_string(input.clone()).expect("success");

    let mut best = f64::MIN;
    let mut best_string = String::new();
    for b in 0b0000_0000..=0b1111_1111 {
        let new_bytes = input_bytes.xor(b);
        let score = new_bytes.valid_ascii_score();
        let ascii = new_bytes.to_ascii();
        if score > best {
            best = score;
            best_string = ascii;
        }
    }

    println!("Decoded {} to {}", input, best_string);
    assert_eq!(best_string, "Cooking MC's like a pound of bacon")
}
