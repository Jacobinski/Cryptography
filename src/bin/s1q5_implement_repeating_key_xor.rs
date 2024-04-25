fn encrypt(input: &[u8], key: &[u8]) -> Vec<u8> {
    let mut encrypted = Vec::new();
    let mut k = 0;
    for i in input {
        encrypted.push(i ^ key[k]);
        k = (k + 1) % key.len();
    }
    encrypted
}
fn main() {
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let input_bytes = input.as_bytes();
    let key = "ICE".as_bytes();
    let encrypted_bytes = encrypt(input_bytes, key);
    let encrypted = hex::encode(encrypted_bytes);

    assert_eq!(
        encrypted,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    )
}
