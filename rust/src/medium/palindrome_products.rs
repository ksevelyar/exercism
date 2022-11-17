#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    pub fn new(value: u64) -> Option<Palindrome> {
        let reversed: u64 = value
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .ok()?;

        match value == reversed {
            true => Some(Self(value)),
            false => None,
        }
    }

    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let palindromes =
        (min..=max).flat_map(|a| (min..=max).flat_map(move |b| Palindrome::new(a * b)));

    let max = palindromes.clone().rev().next()?;
    let min = palindromes.clone().next()?;

    Some((min, max))
}

#[test]
fn test_palindrome_new_return_none() {
    for v in [12, 2322, 23443, 1233211, 8932343] {
        assert_eq!(Palindrome::new(v), None);
    }
}

#[test]
fn test_palindrome_new_return_some() {
    for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
        assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
    }
}

#[test]
fn test_palindrome_10_99() {
    let expected = Some((Palindrome(121), Palindrome(9009)));

    assert_eq!(palindrome_products(10, 99), expected)
}

#[test]
fn test_palindrome_100_999() {
    let expected = Some((Palindrome(10201), Palindrome(906609)));

    assert_eq!(palindrome_products(100, 999), expected)
}

#[test]
fn test_palindrome_1000_9999() {
    let expected = Some((Palindrome(1002001), Palindrome(99000099)));

    assert_eq!(palindrome_products(1000, 9999), expected)
}
