pub fn is_valid(code: &str) -> bool {
    let digits = match parse_input(code) {
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
