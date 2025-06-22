use rand::Rng;
use std::iter::from_fn;

const DEFAULT_KEY_LENGTH: usize = 100;

pub fn encode(key: &str, message: &str) -> Option<String> {
    rotate(key, message, true)
}

pub fn decode(key: &str, message: &str) -> Option<String> {
    rotate(key, message, false)
}

pub fn encode_random(s: &str) -> (String, String) {
    let random_key = random_key(DEFAULT_KEY_LENGTH);
    let encoded = encode(&random_key, s);

    (random_key, encoded.expect("generated key should be valid"))
}

fn ascii_position(ch: char) -> i8 {
    ch as i8 - b'a' as i8
}

fn shifted_char(m: char, k: char, forward: bool) -> Option<char> {
    if !m.is_ascii_lowercase() || !k.is_ascii_lowercase() {
        return None;
    }

    let shift = match forward {
        true => ascii_position(m) + ascii_position(k),
        false => ascii_position(m) - ascii_position(k),
    }
    .rem_euclid(26);

    Some((b'a' + shift as u8) as char)
}

fn rotate(key: &str, message: &str, forward: bool) -> Option<String> {
    if key.is_empty() {
        return None;
    }

    message
        .chars()
        .zip(key.chars().cycle())
        .map(|(message_ch, key_ch)| shifted_char(message_ch, key_ch, forward))
        .collect()
}

fn random_key(len: usize) -> String {
    from_fn(|| Some(rand::thread_rng().gen_range(b'a'..=b'z') as char))
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN_TEXT: &str = "thisismysecret";
    const KEY: &str = "abcdefghij";

    #[test]
    fn cipher_can_encode_with_given_key() {
        assert_eq!(encode(KEY, "aaaaaaaaaa"), Some(KEY.to_string()));
    }

    #[test]
    fn cipher_can_decode_a_message_that_is_shorter_than_the_key() {
        assert_eq!(decode(KEY, "abcde"), Some("aaaaa".to_string()));
    }

    #[test]
    fn cipher_can_double_shift_encode() {
        let plain_text = "iamapandabear";

        assert_eq!(
            encode(plain_text, plain_text),
            Some("qayaeaagaciai".to_string())
        );
    }

    #[test]
    fn cipher_is_reversible_given_key() {
        assert_eq!(
            decode(KEY, &encode(KEY, PLAIN_TEXT).unwrap()),
            Some(PLAIN_TEXT.to_string())
        );
    }

    #[test]
    fn encode_returns_none_with_an_all_caps_key() {
        let key = "ABCDEF";

        assert_eq!(encode(key, PLAIN_TEXT), None);
    }
}
