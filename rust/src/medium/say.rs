pub fn encode(n: u64) -> String {
    let num_string = n.to_string();
    let length = num_string.len() - 1;

    let vec = num_string.chars().rev().collect::<Vec<_>>();
    let thousands = vec.chunks(3).rev().collect::<Vec<_>>();

    thousands
        .into_iter()
        .enumerate()
        .map(|(ind, chars)| {
            say_number(
                chars.iter().rev().copied().collect::<Vec<_>>().as_slice(),
                10u64.pow((length - ind) as u32),
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn say_number(chars: &[char], number: u64) -> String {
    let count = match chars {
        ['1'] => "one".to_string(),
        ['0'] => "zero".to_string(),
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
        ['2', n] => format!("twenty-{}", say_number(&[*n], 0)),
        ['3', n] => format!("thirty-{}", say_number(&[*n], 0)),
        ['4', n] => format!("forty-{}", say_number(&[*n], 0)),
        ['5', n] => format!("fifty-{}", say_number(&[*n], 0)),
        ['6', n] => format!("sixty-{}", say_number(&[*n], 0)),
        ['7', n] => format!("seventy-{}", say_number(&[*n], 0)),
        ['8', n] => format!("eighty-{}", say_number(&[*n], 0)),
        ['9', n] => format!("ninety-{}", say_number(&[*n], 0)),
        ['0', '0'] => format!("").to_string(),
        [n, a, b] => format!(
            "{} hundred {}",
            say_number(&[*n], 100),
            say_number(&[*a, *b], 0)
        ),
        _ => {
            dbg!(chars);
            "dbg!".to_string()
        }
    };

    format!("{}{}", count, say_thouthands(number))
}

fn say_thouthands(count: u64) -> String {
    dbg!(count);
    match count {
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
