pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }

    (0..=(digits.len() - len))
        .map(|ind| digits[ind..(ind + len)].to_string())
        .collect()
}
