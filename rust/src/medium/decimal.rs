use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Decimal {
    a: BigInt,
    b: BigInt,
    b_exp: BigInt,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (a, b, b_exp): (BigInt, BigInt, BigInt) = match input.split_once('.') {
            Some((a, b)) => {
                let exp = b.chars().take_while(|ch| *ch == '0').count();

                (a.parse().ok()?, b.parse().ok()?, exp.into())
            }
            None => (input.parse().ok()?, BigInt::default(), 1.into()),
        };

        Some(Decimal { a, b, b_exp })
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            a: self.a + other.a,
            b: self.b + other.b,
            b_exp: 0.into(),
        }
    }
}

// impl Mul for Decimal {
//     type Output = Self;
//
//     fn mul(self, other: Self) -> Self {
//         Self {
//             a: self.a * other.a,
//             b: self.b * other.b,
//         }
//     }
// }
//
// impl Sub for Decimal {
//     type Output = Self;
//
//     fn sub(self, other: Self) -> Self {
//         Self {
//             a: self.a - other.a,
//             b: self.b - other.b,
//         }
//     }
// }

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
        assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"))
    }

    #[test]
    fn test_sub_borrow() {
        dbg!(decimal("0.0001"));
        // assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
    }

    #[test]
    fn test_add_borrow_integral() {
        assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
    }
}
