// use std::collections::HashMap;

pub fn rotate(input: &str, key: i8) -> String {
    // let alphabet = 'a'..='z';
    let shift = key.rem_euclid(26);

    // let dict: HashMap<char, char> = alphabet
    //     .clone()
    //     .zip(alphabet.cycle().skip(shift as usize))
    //     .collect();
    //
    dbg!(key,shift);

    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphabetic() {
                (ch as u8 + shift as u8) as char
            } else {
                ch
            }
        })
        .collect()
}

#[test]
fn test_rot_negative() {
    assert_eq!(rotate("A", -26), "A".to_owned())
}

#[test]
fn rotate_a_26() {
    assert_eq!("a", rotate("a", 26));
}

#[test]
fn rotate_m_negative_1() {
    assert_eq!("l", rotate("m", -1));
}
