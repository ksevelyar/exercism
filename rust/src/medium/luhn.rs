use core::fmt::Display;

#[derive(Debug)]
pub struct ParseDigitsError(String);

impl std::error::Error for ParseDigitsError {}

impl Display for ParseDigitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't parse to digit: {}", self.0)
    }
}

pub fn is_valid(code: &str) -> bool {
    let digits = match parse_input(code) {
        Ok(chars) => match chars.len() {
          0|1 => return false,
          _ => chars
        },
        Err(_) => return false,
    };

    dbg!(digits);

    true
}

pub fn parse_input(code: &str) -> Result<Vec<u8>, ParseDigitsError> {
    code.chars()
        .filter(|char| !char.is_whitespace())
        .map(|char| {
            char.to_digit(10)
                .ok_or(ParseDigitsError(char.to_string()))
                .map(|x| x as u8)
        })
        .collect()
}
