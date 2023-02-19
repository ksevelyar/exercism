use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use num_bigint::BigInt;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Decimal {
    a: BigInt,
    b: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut v = input.split('.').map(|string| string.parse::<BigInt>().ok());

        Some(Decimal {
            a: v.next()??,
            b: v.next()??,
        })
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            a: self.a * other.a,
            b: self.b * other.b,
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decimal(input: &str) -> Decimal {
        Decimal::try_from(input).expect("That was supposed to be a valid value")
    }

    const BIGS: [&str; 3] = [
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
        "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
        "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
    ];

    #[test]
    fn test_eq() {
        assert!(decimal("0.0") == decimal("0.0"));

        assert!(decimal("1.0") == decimal("1.0"));

        for big in BIGS.iter() {
            assert!(decimal(big) == decimal(big));
        }
    }

    #[test]
    fn test_ne() {
        assert!(decimal("0.0") != decimal("1.0"));

        assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
    }

    #[test]
    fn test_gt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
        }
    }

    #[test]
    fn test_lt() {
        for slice_2 in BIGS.windows(2) {
            assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));

        assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));

        assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
    }
}
