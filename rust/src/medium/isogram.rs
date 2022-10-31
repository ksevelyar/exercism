use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let downcased = candidate.to_lowercase();
    let chars = downcased.chars().filter(|ch| ch.is_ascii_alphabetic());

    let chars_len = chars.clone().count();
    let unuque_chars_len = chars.collect::<HashSet<char>>().len();

    chars_len == unuque_chars_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lamberjacks() {
        assert_eq!(check("Six-year-old"), true);
    }

    #[test]
    fn isogram() {
        assert_eq!(check("isograms"), false);
    }
}
