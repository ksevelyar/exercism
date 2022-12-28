pub struct Luhn<T> {
    input: T,
}

impl<T> Luhn<T>
where
    T: ToString,
{
    pub fn is_valid(&self) -> bool {
        let digits = match Self::parse_input(&self.input.to_string()) {
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
            .map(|(index, digit)| Self::luhn_double(index, *digit))
            .sum();

        sum % 10 == 0
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
}

impl<T> From<T> for Luhn<T> {
    fn from(input: T) -> Luhn<T> {
        Luhn { input }
    }
}

#[test]
fn you_can_validate_from_a_str() {
    let valid = Luhn::from("046 454 286");

    let invalid = Luhn::from("046 454 287");

    assert!(valid.is_valid());

    assert!(!invalid.is_valid());
}

#[test]
fn you_can_validate_from_a_string() {
    let valid = Luhn::from(String::from("046 454 286"));

    let invalid = Luhn::from(String::from("046 454 287"));

    assert!(valid.is_valid());

    assert!(!invalid.is_valid());
}
