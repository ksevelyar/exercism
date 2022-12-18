use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    s.chars()
        .zip(key.chars())
        .map(|(a, b)| {
            let start = b'a';
            let shift = ((a as u8 - start) + (b as u8 - a as u8)).rem_euclid(26);

            if !a.is_ascii_lowercase() || !b.is_ascii_lowercase() {
                return None;
            };

            Some((start + shift as u8) as char)
        })
        .collect()
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    s.chars()
        .zip(key.chars())
        .map(|(a, b)| {
            let start = b'a';
            let shift = ((a as u8 - start) - (b as u8 - a as u8)).rem_euclid(26);

            if !a.is_ascii_lowercase() || !b.is_ascii_lowercase() {
                return None;
            };

            Some((a as u8 - shift as u8) as char)
        })
        .collect()
}

fn random_key(len: usize) -> String {
    (0..len)
        .map(|_| rand::thread_rng().gen_range(b'a'..b'z') as char)
        .collect()
}

pub fn encode_random(s: &str) -> (String, String) {
    let random_key = random_key(s.len());
    let encoded = encode(&random_key, s);

    (random_key, encoded.expect("valid key"))
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
}
