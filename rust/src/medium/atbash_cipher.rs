pub fn encode(plaintext: &str) -> String {
    let is_space_needed = |ind| ind > 0 && ind % 5 == 0;

    plaintext
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(dictionary)
        .enumerate()
        .map(|(ind, ch)| match is_space_needed(ind) {
            true => format!(" {}", ch),
            false => ch.to_string(),
        })
        .collect()
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext
        .chars()
        .filter(|ch| ch.is_alphanumeric())
        .map(dictionary)
        .collect()
}

fn dictionary(ch: char) -> char {
    let alphabet = 'a'..='z';

    if char::is_digit(ch, 10) {
        return ch;
    }

    alphabet
        .clone()
        .zip(alphabet.rev())
        .find(|(in_ch, _out_ch)| *in_ch == ch.to_ascii_lowercase())
        .map(|(_in_ch, out_ch)| out_ch)
        .expect("unknown char")
}

#[test]
fn encode_test() {
    assert_eq!(encode("test"), "gvhg")
}

#[test]
fn decode_test() {
    assert_eq!(encode("gvhg"), "test")
}
