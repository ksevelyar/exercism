const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn starts_with_vowel(input: &str) -> bool {
    let first_char = input.chars().next();
    let second_char = input.chars().nth(1);

    match (first_char, second_char) {
        (Some(char), _) if VOWELS.contains(&char) => true,
        (Some('y'), Some('t')) => true,
        (Some('x'), Some('r')) => true,
        _ => false,
    }
}

fn consonant_sound(input: &str) -> String {
    let split_index = input
        .chars()
        .skip(1)
        .zip(input.chars())
        .position(|(char, next_char)| {
            if char == 'u' && next_char == 'q' {
                return false;
            }

            VOWELS.contains(&char) || char == 'y'
        })
        .unwrap()
        + 1;
    let cluster = input.split_at(split_index).0;
    let word = input.split_at(split_index).1;

    format!("{}{}{}", word, cluster, "ay")
}

fn translate_word(input: &str) -> String {
    if starts_with_vowel(input) {
        return format!("{}{}", input, "ay");
    }

    consonant_sound(input)
}

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

#[test]
fn test_word_beginning_with_a() {
    assert_eq!(translate("apple"), "appleay");
}

#[test]
fn test_word_beginning_with_p() {
    assert_eq!(translate("chair"), "airchay");
}

#[test]
fn test_word_beginning_with_qu_and_a_preceding_consonant() {
    assert_eq!(translate("square"), "aresquay");
}

#[test]
fn test_y_is_treated_like_a_vowel_at_the_end_of_a_consonant_cluster() {
    assert_eq!(translate("rhythm"), "ythmrhay");
}

#[test]
fn test_word_beginning_with_q_without_a_following_u() {
    assert_eq!(translate("qat"), "atqay");
}

#[test]
fn test_word_beginning_with_thr() {
    assert_eq!(translate("thrush"), "ushthray");
}

#[test]
fn test_a_whole_phrase() {
    assert_eq!(translate("quick fast run"), "ickquay astfay unray");
}
