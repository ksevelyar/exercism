use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let num_string = self.0.to_string();
        let length = num_string.len() - 1;
        let roman: String = num_string
            .char_indices()
            .map(|(ind, ch)| {
                Roman::overload(
                    ch.to_digit(10).expect("u32"),
                    10u32.pow((length - ind) as u32),
                )
            })
            .collect();

        write!(f, "{}", roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}

impl Roman {
    const LETTERS: [(&'static str, u32); 7] = [
        ("I", 1),
        ("V", 5),
        ("X", 10),
        ("L", 50),
        ("C", 100),
        ("D", 500),
        ("M", 1000),
    ];

    fn letter(factor: u32) -> &'static str {
        Roman::LETTERS
            .iter()
            .find(|(_roman, arabic)| *arabic == factor)
            .map(|x| x.0)
            .unwrap()
    }

    fn overload(count: u32, factor: u32) -> String {
        match count {
            1..=3 => Roman::letter(factor).repeat(count as usize),
            4 => format!("{}{}", Roman::letter(factor), Roman::letter(factor * 5)),
            5 => Roman::letter(factor * 5).to_string(),
            6..=8 => format!(
                "{}{}",
                Roman::letter(factor * 5),
                Roman::letter(factor).repeat((count - 5) as usize)
            ),
            9 => format!("{}{}", Roman::letter(factor), Roman::letter(factor * 10)),
            _ => "".to_string(),
        }
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

#[test]
fn test_six() {
    assert_eq!("VI", Roman::from(6).to_string());
}

#[test]
fn test_48() {
    assert_eq!("XLVIII", Roman::from(48).to_string());
}

#[test]
fn test_1990() {
    assert_eq!("MCMXC", Roman::from(1990).to_string());
}
