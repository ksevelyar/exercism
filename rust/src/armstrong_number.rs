pub fn check(number: u32) {
    let digits: Vec<_> = number
        .to_string()
        .chars()
        .map(|char| char.to_digit(10).unwrap())
        .collect();

    dbg!(digits);
}
