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

fn combinations(n: usize) -> impl Iterator<Item = Vec<u8>> {
    let init: Vec<_> = (0..=9).take(n).collect();

    successors(Some(init), move |v| {
        let mut v = v.to_vec();

        let index = v.iter().rposition(|&num| num < 9)?;
        v[index] += 1;

        Some(
            v.into_iter()
                .enumerate()
                .map(|(ind, num)| match ind > index {
                    true => 0,
                    false => num,
                })
                .collect(),
        )
    })
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::build(input)?;

    let chars: HashSet<char> = input
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .collect();

    let solution = combinations(chars.len()).find(|set| {
        if set.len() != set.iter().cloned().collect::<HashSet<u8>>().len() {
            return false;
        }

        let map: HashMap<char, u8> = chars.iter().cloned().zip(set.iter().cloned()).collect();

        let sum: usize = puzzle
            .sum
            .iter()
            .map(|term| {
                term.iter().enumerate().fold(0, |acc, (ind, x)| {
                    acc + (map[x] as usize) * (10usize.pow(ind as u32))
                })
            })
            .sum();

        let result = puzzle
            .result
            .chars()
            .rev()
            .enumerate()
            .fold(0, |acc, (ind, x)| {
                acc + (map[&x] as usize) * (10usize.pow(ind as u32))
            });

        sum == result
    })?;

    Some(
        chars
            .iter()
            .cloned()
            .zip(solution.iter().cloned())
            .collect(),
    )
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

    #[test]
    fn test_puzzle_with_ten_letters_and_199_addends() {
        assert_alphametic_solution_eq(
            "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
            &[
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9)
            ],
        );
    }
}
