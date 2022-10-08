fn is_new_word(char: char, next_char: char) -> bool {
    next_char.is_uppercase() && !char.is_uppercase()
        || next_char.is_alphabetic() && !char.is_alphabetic() && char != '\''
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(String::new(), |mut abbr, chars_window| {
            let char = chars_window[0];
            let next_char = chars_window[1];

            if abbr.is_empty() {
                abbr.push(char)
            };

            if is_new_word(char, next_char) {
                abbr.push(next_char)
            };

            abbr
        })
        .to_uppercase()
}

#[test]
fn camelcase() {
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML")
}

#[test]
fn apostrophes() {
    assert_eq!(abbreviate("Halley's Comet"), "HC")
}

#[test]
fn underscore_emphasis() {
    assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT")
}
