#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| encode_char(ch, a, b))
        .enumerate()
        .map(|(ind, ch)| match ind > 0 && ind % 5 == 0 {
            true => format!(" {}", ch),
            false => ch.to_string(),
        })
        .collect())
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(ciphertext
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(|ch| decode_char(ch, a, b))
        .collect())
}

fn is_coprime(a: i32, m: i32) -> bool {
    match a {
        0 => a == 1 || m == 1,
        _ => match a > m {
            true => is_coprime(a.rem_euclid(m), a),
            false => is_coprime(m % a, a),
        },
    }
}

fn encode_char(ch: char, a: i32, b: i32) -> char {
    let alphabet = ('a'..='z').enumerate();

    if char::is_digit(ch, 10) {
        return ch;
    }

    let index = alphabet
        .clone()
        .find(|(_a, b)| *b == ch.to_ascii_lowercase())
        .map(|(a, _b)| a)
        .expect("unknown char");

    let shift = (a as usize * index + b as usize) % 26;

    alphabet
        .clone()
        .find(|(ind, _b)| *ind == shift)
        .map(|(_a, b)| b)
        .expect("unknown index")
}

fn modular_multiplicative_inverse(a: i32) -> i32 {
    let m = 26;

    (1..m).find(|n| (n * a) % m == 1).unwrap()
}

fn decode_char(ch: char, a: i32, b: i32) -> char {
    let alphabet = ('a'..='z').enumerate();

    if char::is_digit(ch, 10) {
        return ch;
    }

    let index = alphabet
        .clone()
        .find(|(_a, b)| *b == ch.to_ascii_lowercase())
        .map(|(a, _b)| a)
        .expect("unknown char");

    let shift = (modular_multiplicative_inverse(a) * (index as i32 - b)).rem_euclid(26);
    dbg!(shift);

    alphabet
        .clone()
        .find(|(ind, _b)| *ind == shift as usize)
        .map(|(_a, b)| b)
        .expect("unknown index")
}

#[test]
fn encode_yes() {
    assert_eq!(encode("yes", 5, 7).unwrap(), "xbt")
}

#[test]
fn encode_all_the_letters() {
    assert_eq!(
        encode("The quick brown fox jumps over the lazy dog.", 17, 33).unwrap(),
        "swxtj npvyk lruol iejdc blaxk swxmh qzglf"
    )
}

#[test]
fn encode_numbers() {
    assert_eq!(
        encode("Testing,1 2 3, testing.", 3, 4).unwrap(),
        "jqgjc rw123 jqgjc rw"
    )
}

#[test]
fn decode_exercism() {
    assert_eq!(decode("tytgn fjr", 3, 7).unwrap(), "exercism")
}

#[test]
fn decode_with_a_not_coprime_to_m() {
    const EXPECTED_ERROR: AffineCipherError = AffineCipherError::NotCoprime(13);

    match decode("Test", 13, 11) {
        Err(EXPECTED_ERROR) => (),

        Err(e) => panic!(
            "Incorrect error: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, e
        ),

        Ok(r) => panic!(
            "Cannot encode/decode when a is coprime to m: expected: {:?}, actual: {:?}.",
            EXPECTED_ERROR, r
        ),
    }
}
