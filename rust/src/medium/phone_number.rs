pub fn number(user_number: &str) -> Option<String> {
    let chars: Vec<char> = user_number
        .chars()
        .enumerate()
        .filter(|(ind, ch)| ch.is_digit(10))
        .map(|(_, ch)| ch)
        .collect();

    dbg!(&chars);

    if chars.len() < 10 || chars.len() > 11 {
        return None;
    }

    if chars.len() == 11 {
        if chars[0] != '1' {
            return None;
        }
    }

    Some(chars.iter().cloned().collect())
}

#[test]
fn test_cleans_the_number() {
    assert_eq!(number("+ 1 (223) 456-7890"), Some("2234567890".to_string()));
}

#[test]
fn test_cleans_the_number_with_multiple_spaces() {
    assert_eq!(number("223 456   7890"), Some("2234567890".to_string()));
}

#[test]
fn test_invalid_if_area_code_starts_with_1() {
    assert_eq!(number("(123) 456-7890"), None);
}
