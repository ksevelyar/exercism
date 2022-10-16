#[derive(Debug)]
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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn year() -> f64 {
        EARTH_YEAR * 0.2408467
    }
}
impl Planet for Venus {
    fn year() -> f64 {
        EARTH_YEAR * 0.61519726
    }
}
impl Planet for Earth {
    fn year() -> f64 {
        EARTH_YEAR * 1.0
    }
}
impl Planet for Mars {
    fn year() -> f64 {
        EARTH_YEAR * 1.8808158
    }
}
impl Planet for Jupiter {
    fn year() -> f64 {
        EARTH_YEAR * 11.862615
    }
}
impl Planet for Saturn {
    fn year() -> f64 {
        EARTH_YEAR * 29.447498
    }
}
impl Planet for Uranus {
    fn year() -> f64 {
        EARTH_YEAR * 84.016846
    }
}
impl Planet for Neptune {
    fn year() -> f64 {
        EARTH_YEAR * 164.79132
    }
}

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
