use std::fmt::format;

pub fn encode(n: u64) -> String {
    let num_string = n.to_string();
    let length = num_string.len() - 1;

    if n > 100 {
    } else {
    }

    num_string
        .char_indices()
        .map(|(ind, ch)| {
            say_factor(
                ch.to_digit(10).expect("u32").into(),
                10u64.pow((length - ind) as u32),
            )
        })
        .collect()
}

fn say_number(number: u64) -> String {
    match number {
        1 => "one".to_string(),
        10 => "ten".to_string(),
        100 => "hundred".to_string(),
        1000 => "thousand".to_string(),
        1000000 => "million".to_string(),
        1000_000_000 => "billion".to_string(),
        1000_000_000_000 => "trillion".to_string(),
        0 => "zero".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        21..=29 => format!("twenty-{}", number - 20),
        30 => "thirty".to_string(),
        31..=39 => format!("thirty-"),
        40 => "forty".to_string(),
        41..=49 => format!("forty-{}", number - 40),
        50 => "fifty".to_string(),
        51..=59 => format!("fifty-{}", number - 50),
        60 => "sixty".to_string(),
        61..=69 => format!("sixty-{}", number - 60),
        70 => "seventy".to_string(),
        71..=79 => format!("seventy-{}", number - 70),
        80 => "eighty".to_string(),
        81..=89 => format!("eighty-{}", number - 80),
        90 => "ninety".to_string(),
        91..=99 => format!("ninety-{}", number - 90),
        _ => "🐗".to_string(),
    }
    .to_string()
}

fn say_factor(count: u64, factor: u64) -> String {
    match (count, factor) {
        (_, 10) => say_number(count * factor),
        (_, 1) => format!("-{}", say_number(count)),
        _ => format!("{} {} ", say_number(count), say_number(factor)),
    }
}

#[test]
fn test_zero() {
    assert_eq!(encode(0), String::from("zero"));
}

#[test]
fn test_one() {
    assert_eq!(encode(1), String::from("one"));
}

#[test]
fn test_fourteen() {
    assert_eq!(encode(14), String::from("fourteen"));
}

#[test]
fn test_twenty() {
    assert_eq!(encode(20), String::from("twenty"));
}

#[test]
fn test_twenty_two() {
    assert_eq!(encode(22), String::from("twenty-two"));
}

#[test]
fn test_one_hundred() {
    assert_eq!(encode(100), String::from("one hundred"));
}

#[test]
fn test_one_thousand_two_hundred_thirty_four() {
    assert_eq!(
        encode(1234),
        String::from("one thousand two hundred thirty-four")
    );
}
