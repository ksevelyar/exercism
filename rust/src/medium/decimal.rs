use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Decimal {
    numerator: BigInt,
    denumerator: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (a, b, b_exp): (BigInt, BigInt, BigInt) = match input.split_once('.') {
            Some((a, b)) => {
                let b_exp = b.chars().count();
                let exp = match a {
                    "0" => BigInt::try_from(10u32).unwrap().pow(b_exp as u32),
                    _ => BigInt::try_from(10u32).unwrap().pow(b_exp as u32),
                };

                (a.parse().ok()?, b.parse().ok()?, exp)
            }
            None => (input.parse().ok()?, BigInt::default(), BigInt::default()),
        };

        let sum = match input.starts_with('-') {
            true => (a * b_exp.clone() - b),
            false => (a * b_exp.clone() + b),
        };

        Some(reduce(sum, b_exp))
    }
}

fn reduce(numerator: BigInt, denumerator: BigInt) -> Decimal {
    if numerator.clone() != BigInt::default() && &numerator % 10 == 0.into() {
        reduce(
            numerator / BigInt::try_from(10u32).unwrap(),
            denumerator / BigInt::try_from(10u32).unwrap(),
        )
    } else {
        Decimal {
            numerator,
            denumerator,
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (numerator, denumerator) = match (&self.denumerator, &other.denumerator) {
            (a, b) if a == b => (&self.numerator + &other.numerator, self.denumerator),
            (a, b) if a > b => {
                let k = &self.denumerator / &other.denumerator;
                (self.numerator + other.numerator * k, self.denumerator)
            }
            (_, _) => {
                let k = &other.denumerator / &self.denumerator;
                (self.numerator * k + other.numerator, other.denumerator)
            }
        };

        reduce(numerator, denumerator)
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (numerator, denumerator) = match (&self.denumerator, &other.denumerator) {
            (a, b) if a == b => (&self.numerator + &other.numerator, self.denumerator),
            (a, b) if a > b => {
                let k = &self.denumerator / &other.denumerator;
                (self.numerator + other.numerator * k, self.denumerator)
            }
            (_, _) => {
                let k = &other.denumerator / &self.denumerator;
                (self.numerator * k + other.numerator, other.denumerator)
            }
        };

        reduce(numerator, denumerator)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (numerator, denumerator) = match (&self.denumerator, &other.denumerator) {
            (a, b) if a == b => (&self.numerator - &other.numerator, self.denumerator),
            (a, b) if a > b => {
                let k = &self.denumerator / &other.denumerator;
                (self.numerator - other.numerator * k, self.denumerator)
            }
            (_, _) => {
                let k = &other.denumerator / &self.denumerator;
                (self.numerator * k - other.numerator, other.denumerator)
            }
        };

        reduce(numerator, denumerator)
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

    #[test]
    fn test_add_away_decimal() {
        assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"));
    }

    #[test]
    fn test_add_borrow_integral() {
        assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
    }

    #[test]
    fn test_add_carry_over_negative() {
        assert_eq!(decimal("-1.99") + decimal("-0.01"), decimal("-2.0"))
    }

    #[test]
    fn test_add_into_fewer_digits() {
        assert_eq!(decimal("0.011") + decimal("-0.001"), decimal("0.01"))
    }

    #[test]
    fn test_sub_away_decimal() {
        assert_eq!(decimal("1.1") - decimal("0.1"), decimal("1.0"))
    }

    #[test]
    fn test_sub_borrow() {
        assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
    }
}
