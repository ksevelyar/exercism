pub fn answer(command: &str) -> Option<i32> {
    let words = &command
        .trim_start_matches("What ")
        .trim_end_matches('?')
        .split_whitespace()
        .filter(|w| *w != "by")
        .collect::<Vec<_>>();

    let operations: Option<Vec<(_, _)>> = words
        .chunks(2)
        .map(|chunk| {
            let command = chunk.first()?;
            let number: i32 = chunk.get(1)?.parse().ok()?;

            Some((command, number))
        })
        .collect();

    Some(
        operations?
            .iter()
            .fold(0, |acc, (command, number)| match **command {
                "is" => *number,
                "plus" => acc + number,
                "minus" => acc - number,
                "multiplied" => acc * number,
                "divided" => acc / number,
                _ => 0,
            }),
    )
}

#[test]
fn just_a_number() {
    let command = "What is 5?";

    assert_eq!(Some(5), answer(command));
}

#[test]
fn addition() {
    let command = "What is 1 plus 1?";

    assert_eq!(Some(2), answer(command));
}

#[test]
fn more_addition() {
    let command = "What is 53 plus 2?";

    assert_eq!(Some(55), answer(command));
}

#[test]
fn addition_with_negative_numbers() {
    let command = "What is -1 plus -10?";

    assert_eq!(Some(-11), answer(command));
}

#[test]
fn large_addition() {
    let command = "What is 123 plus 45678?";

    assert_eq!(Some(45_801), answer(command));
}
