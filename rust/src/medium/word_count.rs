use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let lowercased = phrase.to_lowercase();
    let words = lowercased
        .split(|ch: char| ch.is_whitespace() || ch.is_ascii_punctuation() && ch != '\'')
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches('\''));

    words.fold(HashMap::new(), |mut map, word| {
        *map.entry(word.to_owned()).or_insert(0) += 1;
        map
    })
}

#[test]
fn with_apostrophes() {
    let counts = word_count("Joe can't tell between 'large' and large.");

    let result = HashMap::from([
        ("joe".to_owned(), 1),
        ("can't".to_owned(), 1),
        ("tell".to_owned(), 1),
        ("between".to_owned(), 1),
        ("large".to_owned(), 2),
        ("and".to_owned(), 1),
    ]);

    assert_eq!(counts, result);
}
