const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn vowel_sound(input: &str) -> String {
    let first_char = input.chars().nth(0);
    let second_char = input.chars().nth(1);

    let is_vowel_sound = match (first_char, second_char) {
      (Some(char), _) if VOWELS.contains(&char) => true,
      (Some('y'), Some('t')) => true,
      (Some('x'), Some('r')) => true,
      _ => false
    };

    if is_vowel_sound { return format!("{}{}", input, "ay") }
    input.to_owned()
}

fn consonant_sound() -> String {
    "ay".to_owned()
}

pub fn translate(input: &str) -> String {
    vowel_sound(input)
}

#[test]
fn test_word_beginning_with_a() {
    assert_eq!(translate("apple"), "appleay");
}

#[test]
fn test_word_beginning_with_p() {
    assert_eq!(translate("pig"), "igpay");
}
