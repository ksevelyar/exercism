pub struct Duration {
    seconds: u64,
}

const EARTH_YEAR: f64 = 31_557_600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn year() -> f64;

    fn years_during(duration: &Duration) -> f64 {
        duration.seconds as f64 / Self::year()
    }
}

macro_rules! planet {
    ( $t:ident, $k:expr ) => {
        pub struct $t;
        impl Planet for $t {
            fn year() -> f64 {
                EARTH_YEAR * $k
            }
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_in_delta(expected: f64, actual: f64) {
        let diff: f64 = (expected - actual).abs();

        let delta: f64 = 0.01;

        if diff > delta {
            panic!(
                "Your result of {} should be within {} of the expected result {}",
                actual, delta, expected
            )
        }
    }

    #[test]
    fn earth_age() {
        let duration = Duration::from(1_000_000_000);

        assert_in_delta(31.69, Earth::years_during(&duration));
    }
}
