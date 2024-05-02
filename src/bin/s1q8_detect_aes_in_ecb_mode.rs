use std::collections::HashMap;

fn read_ciphertexts(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn number_identical_chunks(data: &[u8], chunk_size: usize) -> u32 {
    let mut collisions: HashMap<&[u8], u32> = HashMap::new();
    for chunk in data.chunks_exact(chunk_size) {
        collisions.entry(chunk).and_modify(|v| *v += 1).or_insert(1);
    }
    *collisions.iter().max_by_key(|v| v.1).unwrap().1
}

fn main() {
    let ciphertexts = read_ciphertexts("./src/bin/s1q8_input.txt");
    let mut max_score = u32::MIN;
    let mut max_cipher = String::new();
    for ct in ciphertexts.iter().cloned() {
        let score = number_identical_chunks(ct.as_bytes(), 16);
        println!("Score for ciphertext {} is {}", &ct[0..8], score);
        if score > max_score {
            max_score = score;
            max_cipher = ct;
        }
    }
    // ECB mode encryptiion produces a lot of collisions due to its
    // deterministic mapping of X bytes ot plaintest to X bytes of ciphertext.
    println!(
        "There are {} collisions for cipher {}",
        max_score,
        &max_cipher[0..8]
    );
}
