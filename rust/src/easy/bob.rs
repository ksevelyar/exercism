fn is_yell(message: &str) -> bool {
    let is_contain_uppercase = message.chars().any(|c| c.is_ascii_uppercase());
    let is_contain_lowercase = message.chars().any(|c| c.is_ascii_lowercase());

    is_contain_uppercase && !is_contain_lowercase
}

pub fn reply(message: &str) -> &str {
    let last_char = message.trim().chars().last();

    match (last_char, is_yell(message)) {
        (Some('?'), false) => "Sure.",
        (Some('?'), true) => "Calm down, I know what I'm doing!",
        (_, true) => "Whoa, chill out!",
        (None, _) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
