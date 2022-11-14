const NANP_LEN: usize = 10;
const ZONE_START: usize = 0;
const SUBSCRIBER_START: usize = 3;
const NOT_ALLOWED_FOR_START: [char; 2] = ['0', '1'];

fn trim_chars(user_number: &str) -> Vec<char> {
    let chars = user_number.chars().filter(|ch| ch.is_digit(10));

    if chars.clone().count() == 11usize && chars.clone().next() == Some('1') {
        chars.skip(1).collect()
    } else {
        chars.collect()
    }
}

fn is_invalid_area_or_subscriber(area_start: char, subscriber_start: char) -> bool {
    NOT_ALLOWED_FOR_START.contains(&area_start) || NOT_ALLOWED_FOR_START.contains(&subscriber_start)
}

pub fn number(user_number: &str) -> Option<String> {
    let chars = trim_chars(user_number);
    let is_invalid = is_invalid_area_or_subscriber(chars[ZONE_START], chars[SUBSCRIBER_START]);

    if chars.len() != NANP_LEN || is_invalid { return None }

    Some(chars.iter().collect())
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

#[test]
fn test_invalid_when_11_digits_does_not_start_with_a_1() {
    assert_eq!(number("22234567890"), None);
}
