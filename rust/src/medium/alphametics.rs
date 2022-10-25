use std::collections::HashMap;

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
            .map(|word| word.chars().collect())
            .collect();

        let result = sum_and_result.get(1)?;

        Some(Puzzle { sum, result })
    }

    fn possible_solutions(self: &Puzzle<'a>) -> std::ops::Range<usize> {
        Puzzle::max_solution(&self.sum)..Puzzle::min_solution(self.result)
    }

    fn max_value_for_length(len: usize) -> usize {
        (0..len).enumerate().fold(0, |acc, (index, _digit)| {
            acc + 9 * 10_usize.pow(index as u32)
        })
    }

    fn max_solution(sum: &[Vec<char>]) -> usize {
        sum.iter()
            .map(|term| Self::max_value_for_length(term.len()))
            .sum()
    }

    fn min_solution(result: &str) -> usize {
        10_usize.pow(result.len() as u32 - 1)
    }
}

fn verify_col(col: &[char], col_sum: u8, known_chars: &HashMap<char, u8>) {}

fn verify_cols(puzzle: &Puzzle, num: usize) -> bool {
    let known_chars: HashMap<char, u8> = puzzle
        .result
        .chars()
        .zip(
            num.to_string()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8),
        )
        .collect();

    dbg!(verify_col(&['o', 'o', 'o'], 2u8, &known_chars));
    false
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::build(input)?;

    dbg!(&puzzle);
    dbg!(&puzzle.possible_solutions());

    dbg!(verify_cols(&puzzle, 108));

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
