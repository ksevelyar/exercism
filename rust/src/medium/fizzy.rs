use std::fmt::Display;
use std::ops::Rem;

pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: Display>(matcher: F, subs: S) -> Matcher<T> {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Display + Copy> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);

        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |elem| {
            let substitutions: String = self
                .matchers
                .iter()
                .filter(|matcher| (matcher.matcher)(elem))
                .map(|matcher| matcher.subs.clone())
                .collect();

            if substitutions.is_empty() {
                return elem.to_string();
            }
            substitutions
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Display + Rem<Output = T> + From<u8> + std::cmp::PartialEq,
{
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(|n| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n| n % T::from(5) == T::from(0), "buzz"))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        () => {
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16",
            ]
        };
    }

    #[test]
    fn test_simple() {
        let got = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();

        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];

        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));

        let got = fizzer.apply(1..=16).collect::<Vec<_>>();

        assert_eq!(expect, got);
    }
}
