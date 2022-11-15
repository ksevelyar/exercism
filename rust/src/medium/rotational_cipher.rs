fn rotate_char(ch: char, key: i8) -> char {
    let start = match ch.is_ascii_uppercase() {
        true => b'A',
        false => b'a',
    };

    let shift = ((ch as u8 - start) as i8 + key).rem_euclid(26);

    (start + shift as u8) as char
}

pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|ch| {
            if !ch.is_ascii_alphabetic() { return ch }
            rotate_char(ch, key)
        })
        .collect()
}

#[test]
fn rotate_a_26() {
    assert_eq!("A", rotate("A", 26));
}

#[test]
fn rotate_a_0() {
    assert_eq!("a", rotate("a", 0));
}

#[test]
fn rotate_m_negative_1() {
    assert_eq!("l", rotate("m", -1));
}

#[test]
fn rotate_n_13_with_wrap() {
    assert_eq!("a", rotate("n", 13));
}
