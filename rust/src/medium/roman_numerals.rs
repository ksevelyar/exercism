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

pub struct Roman;

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        unimplemented!("Return a roman-numeral string representation of the Roman object");
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {}
}
