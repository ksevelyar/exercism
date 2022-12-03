use std::fmt::{Display, Formatter, Result};

const ROMAN_ARABIC: [(char, u32); 7] = [
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000),
];

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let roman = ROMAN_ARABIC
            .iter()
            .filter(|(_roman, arabic)| num % arabic == 0)
            .map(|(roman, arabic)| roman.to_string().repeat((num / arabic) as usize))
            .collect();

        Roman(roman)
    }
}

#[test]
fn test_three() {
    assert_eq!("III", Roman::from(3).to_string());
}

#[test]
fn test_four() {
    assert_eq!("IV", Roman::from(4).to_string());
}
