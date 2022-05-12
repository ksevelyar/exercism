pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    let pow = string.len() as u32;

    let sum: u32 = string
        .chars()
        .map(|char| char.to_digit(10).unwrap().pow(pow))
        .sum();

    sum == num
}
