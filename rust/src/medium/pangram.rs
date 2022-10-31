pub fn is_pangram(sentence: &str) -> bool {
    let chars: Vec<char> = sentence.to_lowercase().chars().collect();

    ('a'..='z').all(|alphabet_char| chars.contains(&alphabet_char))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_pangram_is_a_pangram() {
        let sentence = "the quick brown fox jumps over the lazy dog";

        assert!(is_pangram(sentence));
    }
}
