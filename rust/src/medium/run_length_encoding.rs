pub fn encode(source: &str) -> String {
    let tuples = source
        .chars()
        .zip(source.chars().cycle().skip(1))
        .enumerate()
        .filter(|(_, (a, b))| a != b)
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
    unimplemented!("Return the run-length decoding of {}.", source);
}

#[test]
fn test_encode_string_with_no_single_characters() {
    assert_eq!("2A3B4C", encode("AABBBCCCC"));
}
