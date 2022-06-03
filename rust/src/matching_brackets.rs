fn is_symmetry(string: &str) -> bool {
    let unbalanced_brackets = string.chars().fold(
        (0, 0, 0),
        |(parenthesis, brace, bracket), char| match char {
            '(' if parenthesis >= 0 => (parenthesis + 1, brace, bracket),
            '{' if brace >= 0 => (parenthesis, brace + 1, bracket),
            '[' if bracket >= 0 => (parenthesis, brace, bracket + 1),

            ')' => (parenthesis - 1, brace, bracket),
            '}' => (parenthesis, brace - 1, bracket),
            ']' => (parenthesis, brace, bracket - 1),

            _ => (parenthesis, brace, bracket),
        },
    );

    unbalanced_brackets == (0, 0, 0)
}

fn is_wrong_pattern(string: &str) -> bool {
    let brackets = ['(', ')', '[', ']', '{', '}'];
    let input_brackets = string.chars().filter(|char| brackets.contains(char));
    // FIXME: remove mutation and clone
    let mut paired_input_brackets = input_brackets.clone().zip(input_brackets.skip(1));
    // FIXME: extract to external function
    paired_input_brackets.any(|(prev, current)| match (prev, current) {
        ('(', ']') => true,
        ('(', '}') => true,

        ('[', ')') => true,
        ('[', '}') => true,

        ('{', ']') => true,
        ('{', ')') => true,

        _ => false,
    })
}

pub fn brackets_are_balanced(string: &str) -> bool {
    !is_wrong_pattern(string) && is_symmetry(string)
}
