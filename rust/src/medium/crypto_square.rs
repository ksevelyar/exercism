pub fn encrypt(input: &str) -> String {
    let chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect();

    let count = chars.len() as i32;

    let (c, r) = (1..count)
        .flat_map(|c| (1..count).map(move |r| (c, r)))
        .find(|(c, r)| ((c - r) <= 1) && c >= r && c * r >= count)
        .unwrap();

    let rows = chars.chunks(c as usize).collect::<Vec<_>>();

    dbg!(rows);
    todo!()
}

#[test]
fn test_example() {
    assert_eq!(
        encrypt("If man was meant to stay on the ground, god would have given us roots."),
        "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
    )
}
