use std::collections::HashSet;

fn sorted_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();

    chars
}

fn is_anagram(word: &str, candidate: &str) -> bool {
    let downcased_candidate = candidate.to_lowercase();
    if word == downcased_candidate {
        return false;
    }

    sorted_chars(word) == sorted_chars(&downcased_candidate)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let downcased_word = word.to_lowercase();

    possible_anagrams
        .iter()
        .filter(|candidate| is_anagram(&downcased_word, candidate))
        .copied()
        .collect()
}
