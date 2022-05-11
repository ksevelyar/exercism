pub fn check(number: u32) -> bool {
    let chars: Vec<_> = number
        .to_string()
        .chars()
        .collect();

    let pow = chars.len();

    let digits: Vec<_> = chars.iter()
        .map(| char| char.to_digit(10).unwrap().pow(pow as u32))
        .collect();

    let sum: u32 = digits.iter().sum();

    sum == number
}
