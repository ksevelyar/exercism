pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(s1.chars().zip(s2.chars()).fold(0, |acc, (a, b)| {
        if a != b {
            return acc + 1;
        }
        acc
    }))
}

#[test]
fn test_distance() {
    assert_eq!(hamming_distance("ACCAGGG", "ACTATGG"), Some(2))
}
