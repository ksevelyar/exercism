pub trait Luhn<T>: ToString {
    fn valid_luhn(&self) -> bool;
}

fn luhn_double(index: usize, digit: u32) -> u32 {
    let processed_digit = match index % 2 {
        0 => return digit,
        _ => digit * 2,
    };

    match processed_digit > 9 {
        true => processed_digit - 9,
        false => processed_digit,
    }
}

fn parse_input(code: &str) -> Option<Vec<u32>> {
    code.chars()
        .filter(|char| !char.is_whitespace())
        .map(|char| char.to_digit(10))
        .collect()
}

impl<T: std::fmt::Display> Luhn<T> for T {
    fn valid_luhn(&self) -> bool {
        let digits = match parse_input(&self.to_string()) {
            Some(chars) => match chars.len() {
                0 | 1 => return false,
                _ => chars,
            },
            None => return false,
        };

        let sum: u32 = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, digit)| luhn_double(index, *digit))
            .sum();

        sum % 10 == 0
    }
}

#[test]
fn you_can_validate_from_a_str() {
    assert!("046 454 286".valid_luhn());

    assert!(!"046 454 287".valid_luhn());
}

#[test]
fn you_can_validate_from_a_string() {
    assert!(String::from("046 454 286").valid_luhn());

    assert!(!String::from("046 454 287").valid_luhn());
}

#[test]
fn you_can_validate_from_a_u8() {
    assert!(240u8.valid_luhn());

    assert!(!241u8.valid_luhn());
}
