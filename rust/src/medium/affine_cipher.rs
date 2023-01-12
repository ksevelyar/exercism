#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn encode_char(ch: char, a: i32, b: i32) -> char {
    let alphabet = ('a'..'z').enumerate();

    let index = alphabet
        .clone()
        .find(|(_a, b)| *b == ch)
        .map(|(a, _b)| a)
        .expect("unknown alphabet");

    let shift = (a as usize * index + b as usize) % 26;

    alphabet
        .clone()
        .find(|(ind, _b)| *ind == shift)
        .map(|(_a, b)| b)
        .expect("unknown alphabet")
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    Ok(plaintext.chars().map(|ch| encode_char(ch, a, b)).collect())
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}

#[test]
fn encode_yes() {
    assert_eq!(encode("yes", 5, 7).unwrap(), "xbt")
}
