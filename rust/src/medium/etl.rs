use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(score, chars)| chars.iter().map(|ch| (ch.to_ascii_lowercase(), *score)))
        .collect()
}
