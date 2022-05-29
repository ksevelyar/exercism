fn is_start_bracket(char: &char) -> bool {
    match char {
        '(' | '{' | '[' => true,
        _ => false,
    }
}

fn is_end_bracket(char: &char) -> bool {
    match char {
        ')' | '}' | ']' => true,
        _ => false,
    }
}

fn compare_brackets(start_brackets: Vec<char>, end_brackets: Vec<char>) -> bool {
    if start_brackets.len() == 0 || start_brackets.len() != end_brackets.len() {
        return false;
    }

    let end_brackets_reverted = end_brackets.iter().rev().map(|bracket| match bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("not a bracket"),
    });

    start_brackets
        .iter()
        .zip(end_brackets_reverted)
        .all(|(a, b)| a == &b)
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let start_brackets = string
        .chars()
        .filter(|char| is_start_bracket(char))
        .collect();
    let end_brackets = string.chars().filter(|char| is_end_bracket(char)).collect();

    compare_brackets(start_brackets, end_brackets)
}
