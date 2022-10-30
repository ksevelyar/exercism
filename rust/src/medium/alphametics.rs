use std::collections::{HashMap, HashSet};

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

fn combinations(items: &[u8], n: usize, acc: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let acc = match acc.is_empty() {
        false => acc,
        true => items.iter().cloned().map(|item| vec![item]).collect(),
    };

    if n <= 1 {
        return acc;
    };

    let new_acc: Vec<Vec<u8>> = items
        .iter()
        .flat_map(|x| {
            acc.iter().filter(|set| !set.contains(x)).map(|set| {
                let mut new_set = set.clone();
                new_set.push(*x);

                new_set
            })
        })
        .collect();

    combinations(items, n - 1, new_acc)
}

fn known_chars(puzzle: &Puzzle, num: usize) -> HashMap<char, u8> {
    puzzle
        .result
        .chars()
        .zip(
            num.to_string()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8),
        )
        .collect()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::build(input)?;

    let first_column: Vec<char> = puzzle.sum.iter().map(|term| term[0]).collect();
    let chars: HashSet<char> = puzzle.sum.iter().map(|term| term[0]).collect();

    let first_col_result = puzzle.result.chars().rev().collect::<Vec<_>>()[0];

    dbg!(&chars);

    let combinations = combinations(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], chars.len(), Vec::new());
    let combinations_f = combinations
        .iter()
        .filter(|set| {
            let map: HashMap<char, u8> = chars.iter().cloned().zip(set.iter().cloned()).collect();
            let sum: usize = first_column.iter().map(|x| map[x] as usize).sum();

            (sum % 10) == map[&first_col_result] as usize
        })
        .cloned()
        .collect::<Vec<Vec<u8>>>();

    dbg!(combinations_f.len());

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
