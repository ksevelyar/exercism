use std::collections::HashMap;

#[derive(Debug)]
pub struct Puzzle<'a> {
  sum: Vec<&'a str>,
  target: &'a str
}

pub fn parse(input: &str) -> Option<Puzzle> {
    let sum_and_target: Vec<&str> = input.split(" == ").collect();

    let sum = sum_and_target.get(0)?.split(" + ").collect();
    let target = sum_and_target.get(1)?;

    Some(Puzzle { sum, target })
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let sum = input.split("==").last()?;
    dbg!(sum);

    unimplemented!("Solve the alphametic {:?}", input)
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
