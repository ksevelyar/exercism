pub fn encode(n: u64) -> String {
    let num_string = n.to_string();
    let length = num_string.len() - 1;

    let vec = num_string.chars().rev().collect::<Vec<_>>();
    let thousands = vec.chunks(3).rev().collect::<Vec<_>>();

    thousands
        .into_iter()
        .enumerate()
        .map(|(ind, chars)| {
            let chars = chars.iter().rev().copied().collect::<Vec<_>>();
            let power = 10u64.pow((length - ind) as u32);

            dbg!(&chars);
            match chars.as_slice() {
                [a, '0', '0'] => format!("{}{}", say_number(&[*a]), say_thousands(power)),
                [a, b, c] => format!(
                    "{}{} {}",
                    say_number(&[*a]),
                    say_thousands(power),
                    say_number(&[*b, *c])
                ),
                ['0'] => "zero".to_string(),
                _ => format!("{}{}", say_number(&chars), say_thousands(power)),
            }
        })
        .filter(|x| dbg!(x) != " " && x != "")
        .collect::<Vec<_>>()
        .join(" ")
}

fn say_number(chars: &[char]) -> String {
    match chars {
        ['1'] => "one".to_string(),
        ['2'] => "two".to_string(),
        ['3'] => "three".to_string(),
        ['4'] => "four".to_string(),
        ['5'] => "five".to_string(),
        ['6'] => "six".to_string(),
        ['7'] => "seven".to_string(),
        ['8'] => "eight".to_string(),
        ['9'] => "nine".to_string(),
        ['1', '0'] => "ten".to_string(),
        ['1', '1'] => "eleven".to_string(),
        ['1', '2'] => "twelve".to_string(),
        ['1', '3'] => "thirteen".to_string(),
        ['1', '4'] => "fourteen".to_string(),
        ['1', '5'] => "fifteen".to_string(),
        ['1', '6'] => "sixteen".to_string(),
        ['1', '7'] => "seventeen".to_string(),
        ['1', '8'] => "eighteen".to_string(),
        ['1', '9'] => "nineteen".to_string(),
        ['2', '0'] => "twenty".to_string(),
        ['2', n] => format!("twenty-{}", say_number(&[*n])),
        ['3', n] => format!("thirty-{}", say_number(&[*n])),
        ['4', n] => format!("forty-{}", say_number(&[*n])),
        ['5', n] => format!("fifty-{}", say_number(&[*n])),
        ['6', n] => format!("sixty-{}", say_number(&[*n])),
        ['7', n] => format!("seventy-{}", say_number(&[*n])),
        ['8', n] => format!("eighty-{}", say_number(&[*n])),
        ['9', n] => format!("ninety-{}", say_number(&[*n])),
        ['0', '0'] => "".to_string(),
        ['0', n] => say_number(&[*n]),
        _ => format!(""),
    }
}

fn say_thousands(count: u64) -> String {
    match count {
        100 => " hundred".to_string(),
        1000 => " thousand".to_string(),
        1000_000 => " million".to_string(),
        1000_000_000 => " billion".to_string(),
        1000_000_000_000 => " trillion".to_string(),
        _ => "".to_string(),
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

#[test]
fn test_one_million_two() {
    assert_eq!(encode(1_000_002), String::from("one million two"));
}
