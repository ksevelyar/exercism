pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (1..=start_bottles)
        .rev()
        .take(take_down as usize)
        .map(|i| make_verse(i))
        .collect::<Vec<_>>()
        .join("\n")
}

fn make_verse(i: u32) -> String {
    let amount = index_to_literal(i);
    let amount_minus_one = index_to_literal(i - 1).to_lowercase();

    format!(
        "
{amount} hanging on the wall,
{amount} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {amount_minus_one} hanging on the wall."
    )
}

fn index_to_literal(i: u32) -> &'static str {
    match i {
        0 => "no green bottles",
        1 => "One green bottle",
        2 => "Two green bottles",
        3 => "Three green bottles",
        4 => "Four green bottles",
        5 => "Five green bottles",
        6 => "Six green bottles",
        7 => "Seven green bottles",
        8 => "Eight green bottles",
        9 => "Nine green bottles",
        10 => "Ten green bottles",
        _ => panic!(),
    }
}
