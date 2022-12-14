pub fn encode(source: &str) -> String {
    let tuples = source
        .chars()
        .zip(source.chars().cycle().skip(1))
        .enumerate()
        .filter(|(ind, (a, b))| a != b || *ind == source.chars().count() - 1)
        .map(|(ind, (a, _))| (ind, a))
        .fold(Vec::new(), |mut acc, (ind, ch)| {
            let sum: usize = acc.iter().map(|(_, num)| num).sum();

            acc.push((ch, ind + 1 - sum));
            acc
        });

    tuples
        .iter()
        .map(|(ch, repeat)| match repeat {
            1 => ch.to_string(),
            _ => format!("{}{}", repeat, ch),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    source
        .split_inclusive(|ch: char| ch.is_alphabetic() || ch.is_whitespace())
        .map(|str| {
            str.rsplitn(3, "")
                .filter(|str| !str.is_empty())
                .collect::<Vec<_>>()
        })
        .map(|vec| match vec[..] {
            [a, b] => a.to_string().repeat(b.parse().unwrap()),
            [a] => a.to_string(),
            _ => "".to_string(),
        })
        .collect()
}

#[test]
fn test_encode_string_with_no_single_characters() {
    assert_eq!("2A3B4C", encode("AABBBCCCC"));
}

#[test]
fn test_decode_string_with_no_single_characters() {
    assert_eq!("AABBBCCCC", decode("2A3B4C"));
}

#[test]
fn test_decode_multiple_whitespace_mixed_in_string() {
    assert_eq!("  hsqq qww  ", decode("2 hs2q q2w2 "));
}

#[test]
fn test_encode_multiple_whitespace_mixed_in_string() {
    assert_eq!("2 hs2q q2w2 ", encode("  hsqq qww  "));
}
