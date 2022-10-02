fn no_more_bottles() -> String {
    "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
"
    .to_string()
}

fn bottles(current: String, left: String, step: &str) -> String {
    format!(
        "{current} of beer on the wall, {current} of beer.
Take {step} down and pass it around, {left} of beer on the wall.
"
    )
}

pub fn verse(n: u32) -> String {
    match n {
        3.. => {
            let next_n = n - 1;
            bottles(format!("{n} bottles"), format!("{next_n} bottles"), "one")
        }
        2 => bottles("2 bottles".to_string(), "1 bottle".to_string(), "one"),
        1 => bottles("1 bottle".to_string(), "no more bottles".to_string(), "it"),
        0 => no_more_bottles(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
