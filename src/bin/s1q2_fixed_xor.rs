use core::fmt;

struct Bytes {
    data: Vec<u8>,
}

#[derive(Debug)]
struct InvalidInput;

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://stackoverflow.com/a/54044287
        for (i, val) in self.data.iter().enumerate() {
            if i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{:08b}", val)?;
        }
        Ok(())
    }
}

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

    fn xor(a: Bytes, b: Bytes) -> Bytes {
        let mut data: Vec<u8> = Vec::new();
        let iter = std::iter::zip(a.data.into_iter(), b.data.into_iter());
        for (x, y) in iter {
            let z = x ^ y;
            data.push(z);
        }
        Bytes { data }
    }

    fn to_hex(self) -> String {
        let mut fragments: Vec<String> = Vec::new();
        for b in self.data.iter() {
            fragments.push(format!("{:x}", b))
        }
        fragments.join("")
    }
}

fn main() {
    let input1 = String::from("1c0111001f010100061a024b53535009181c");
    let input2 = String::from("686974207468652062756c6c277320657965");
    let output = String::from("746865206b696420646f6e277420706c6179");

    let b1 = Bytes::from_hex_string(input1).expect("success");
    let b2 = Bytes::from_hex_string(input2).expect("success");
    let x = Bytes::xor(b1, b2);
    let res = x.to_hex();

    assert_eq!(res, output);
    println!("Got result {}", res);
}
