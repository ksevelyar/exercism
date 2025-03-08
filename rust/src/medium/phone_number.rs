const NANP_LEN: usize = 10;
const NOT_ALLOWED_FOR_START: [char; 2] = ['0', '1'];

fn trim_country_code_and_non_digits(phone: &str) -> String {
    let chars = phone.chars().filter(|ch| ch.is_ascii_digit());

    let is_phone_starts_with_country_code =
        chars.clone().next() == Some('1') && chars.clone().count() == 11;

    if is_phone_starts_with_country_code {
        chars.skip(1).collect()
    } else {
        chars.collect()
    }
}

fn validate(phone: &str) -> Option<()> {
    if phone.len() != NANP_LEN {
        return None;
    }

    let zone_start = phone.chars().next()?;
    let local_number_start = phone.chars().nth(3)?;

    if NOT_ALLOWED_FOR_START.contains(&zone_start)
        || NOT_ALLOWED_FOR_START.contains(&local_number_start)
    {
        return None;
    }

    Some(())
}

pub fn number(phone: &str) -> Option<String> {
    let phone = trim_country_code_and_non_digits(phone);

    validate(&phone)?;

    Some(phone)
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
