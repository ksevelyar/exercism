use std::collections::{HashMap, HashSet};
use std::iter::successors;

#[derive(Debug)]
struct Puzzle<'a> {
    sum: Vec<Vec<char>>,
    result: &'a str,
}

impl<'a> Puzzle<'a> {
    fn build(input: &str) -> Option<Puzzle> {
        let sum_and_result: Vec<&str> = input.split(" == ").collect();

        let sum = sum_and_result
            .get(0)?
            .split(" + ")
            .map(|word| word.chars().rev().collect())
            .collect();

        let result = sum_and_result.get(1)?;

        Some(Puzzle { sum, result })
    }
}

fn combinations(n: usize) -> impl Iterator<Item = Vec<usize>> {
    // n possibilities
    let range = 0..=9;

    let init: Vec<_> = (0..=9).take(n).collect();
    dbg!(&init);

    successors(Some(init), move |v| {
        let mut v = v.to_vec();

        v[n - 1] += 1;

        Some(v)
    })
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::build(input)?;

    let chars: HashSet<char> = input
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .collect();

    dbg!(&chars);

    let combinations: Vec<_> = combinations(chars.len()).take(10).collect();
    dbg!(combinations);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
        let answer = solve(puzzle);

        let solution: HashMap<char, u8> = solution.iter().cloned().collect();

        assert_eq!(answer, Some(solution));
    }

    #[test]
    fn test_three_letters() {
        assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
    }

    #[test]
    fn test_six_letters() {
        assert_alphametic_solution_eq(
            "NO + NO + TOO == LATE",
            &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
        );
    }
}
