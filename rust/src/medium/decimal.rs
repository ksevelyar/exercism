use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Decimal {
    a: BigInt,
    b: BigInt,
    b_exp: i32,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (a, b, b_exp): (BigInt, BigInt, i32) = match input.split_once('.') {
            Some((a, b)) => {
                let b_exp = b.chars().take_while(|ch| *ch == '0').count();
                let exp = match a {
                    "0" => b_exp + 1,
                    _ => b_exp,
                };

                (a.parse().ok()?, b.parse().ok()?, exp as i32)
            }
            None => (input.parse().ok()?, BigInt::default(), 1),
        };

        Some(Decimal { a, b, b_exp })
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let exp_diff = (self.b_exp - other.b_exp).abs();
        let exp = BigInt::try_from(10u32).unwrap().pow(exp_diff as u32);

        let sum = if self.b_exp < other.b_exp {
            (self.a + self.b) * exp.clone() + (other.a * exp.clone() + other.b)
        } else {
            (self.a * exp.clone() + self.b) + (other.a + other.b) * exp.clone()
        };

        let a = sum.clone() / exp.clone();
        let b_exp = match &a {
            x if *x == BigInt::try_from(0u32).unwrap() => exp_diff + 1,
            _ => exp_diff,
        };

        Self {
            a,
            b: sum % exp,
            b_exp,
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

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let exp_diff = (self.b_exp - other.b_exp).abs();
        let exp = BigInt::try_from(10u32).unwrap().pow(exp_diff as u32);

        let sum = if self.b_exp < other.b_exp {
            (self.a + self.b) * exp.clone() - (other.a * exp.clone() + other.b)
        } else {
            (self.a * exp.clone() + self.b) - (other.a + other.b) * exp.clone()
        };

        let a = sum.clone() / exp.clone();
        let b_exp = match &a {
            x if *x == BigInt::try_from(0u32).unwrap() => exp_diff + 1,
            _ => exp_diff,
        };

        Self {
            a,
            b: sum % exp,
            b_exp,
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

    #[test]
    fn test_add_away_decimal() {
        assert_eq!(decimal("1.1") + decimal("-0.1"), decimal("1.0"))
    }

    #[test]
    fn test_sub_borrow() {
        assert_eq!(decimal("0.01") - decimal("0.0001"), decimal("0.0099"))
    }

    #[test]
    fn test_add_borrow_integral() {
        assert_eq!(decimal("1.0") + decimal("-0.01"), decimal("0.99"))
    }
}
