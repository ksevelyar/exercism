use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let words: Vec<String> = phrase
        .to_lowercase()
        .split(|ch: char| ch.is_whitespace() || ch.is_ascii_punctuation() && ch != '\'')
        .filter(|x| *x != "")
        .map(|x| x.to_owned())
        .collect();

    dbg!(&words);
    HashMap::new()
}

#[test]
fn with_apostrophes() {
    let counts = word_count("First: don't laugh. Then: don't cry.");

    let result = HashMap::from([
        ("first".to_owned(), 1),
        ("don't".to_owned(), 2),
        ("laugh".to_owned(), 1),
        ("then".to_owned(), 1),
        ("cry".to_owned(), 1),
    ]);

    assert_eq!(counts, result);
}
