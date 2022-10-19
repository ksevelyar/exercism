use std::collections::HashMap;

#[derive(Debug)]
pub struct Puzzle<'a> {
    sum: Vec<&'a str>,
    target: &'a str,
}

fn parse(input: &str) -> Option<Puzzle> {
    let sum_and_target: Vec<&str> = input.split(" == ").collect();

    let sum = sum_and_target.get(0)?.split(" + ").collect();
    let target = sum_and_target.get(1)?;

    Some(Puzzle { sum, target })
}

fn max_for_length(len: usize) -> usize {
    (0..len).rev().enumerate().fold(0, |acc, (index, _digit)| {
        acc + 9 * 10_usize.pow(index as u32)
    })
}

fn max_target(sum: &[&str]) -> usize {
    sum.iter().map(|term| max_for_length(term.len())).sum()
}
fn min_target(target: &str) -> usize {
    10_usize.pow(target.len() as u32 - 1)
}
fn verify_guess_for_column() -> bool {
    true
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = parse(input)?;

    dbg!(&puzzle);
    dbg!(max_target(&puzzle.sum));
    dbg!(min_target(&puzzle.target));

    let mut map: HashMap<char, u8> = HashMap::new();
    map.insert('A', 1);
    Some(map)
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
    fn test_with_three_letters() {
        assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
    }
}
