use core::cmp::Ordering;
use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug)]
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
                    "0" => BigInt::from(10u32).pow(b_exp as u32),
                    _ => BigInt::from(10u32).pow(b_exp as u32),
                };

                (a.parse().ok()?, b.parse().ok()?, exp)
            }
            None => (
                input.parse().ok()?,
                BigInt::default(),
                BigInt::from(1u32),
            ),
        };

        let sum = match input.starts_with('-') {
            true => a * b_exp.clone() - b,
            false => a * b_exp.clone() + b,
        };

        Some(reduce(sum, b_exp))
    }
}

fn reduce(numerator: BigInt, denumerator: BigInt) -> Decimal {
    if numerator != BigInt::default()
        && &numerator % 10 == 0.into()
        && &denumerator % 10 == 0.into()
    {
        reduce(
            numerator / BigInt::from(10u32),
            denumerator / BigInt::from(10u32),
        )
    } else {
        Decimal {
            numerator,
            denumerator,
        }
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (a, b) = match (&self.denumerator, &other.denumerator) {
            (a, b) if a == b => (self.numerator.clone(), other.numerator.clone()),
            (a, b) if a > b => {
                let k = &self.denumerator / &other.denumerator;
                (self.numerator.clone(), (&other.numerator * &k))
            }
            (_, _) => {
                let k = &other.denumerator / &self.denumerator;
                (&self.numerator * k, other.numerator.clone())
            }
        };

        Some(a.cmp(&b))
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        match (&self.numerator, &other.numerator) {
            (a, b) if *a == BigInt::default() && *b == BigInt::default() => true,
            (_, _) => self.numerator == other.numerator && self.denumerator == other.denumerator,
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
            (a, b) if a == b => (&self.numerator * &other.numerator, self.denumerator),
            (a, b) if a > b => (
                self.numerator * other.numerator,
                self.denumerator * other.denumerator,
            ),
            (_, _) => (
                self.numerator * other.numerator,
                other.denumerator * self.denumerator,
            ),
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

    #[test]
    fn test_add_vary_precision() {
        assert_eq!(
            decimal("100000000000000000000000000000000000000000000")
                + decimal("0.00000000000000000000000000000000000000001"),
            decimal(BIGS[0])
        )
    }

    #[test]
    fn test_mul_id() {
        assert_eq!(decimal("2.1") * decimal("1.0"), decimal("2.1"));

        assert_eq!(decimal("1.0") * decimal("2.1"), decimal("2.1"));
    }

    #[test]
    fn test_eq_vary_sig_digits() {
        assert!(decimal("0") == decimal("0000000000000.0000000000000000000000"));

        assert!(decimal("1") == decimal("00000000000000001.000000000000000000"));
    }

    #[test]
    fn test_gt_varying_positive_precisions() {
        assert!(decimal("1.1") > decimal("1.01"));

        assert!(decimal("1.01") > decimal("1.0"));

        assert!(decimal("1.0") > decimal("0.1"));

        assert!(decimal("0.1") > decimal("0.01"));
    }

    #[test]
    fn test_gt_negative_and_zero() {
        assert!(decimal("0.0") > decimal("-0.1"));

        assert!(decimal("0.0") > decimal("-1.0"));
    }
}
