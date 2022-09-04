use std::str;

fn annotate_row(index: usize, minefield: &[&str]) -> String {
    let prev_row = index.saturating_sub(1);
    let next_row = match index {
        _ if index == minefield.len() - 1 => index,
        _ => index + 1,
    };

    let rows = minefield.get(prev_row..=next_row).unwrap();

    let v: Vec<&[u8]> = rows.iter().map(|row| row.as_bytes()).collect();
    dbg!(v);

    "String".to_owned()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // let windows = minefield[0].as_bytes();
    // let ascii = dbg!(" * ".as_bytes());
    // let ascii_string = str::from_utf8(ascii).unwrap();
    // vec![ascii_string.to_owned()]

    (0..minefield.len())
        .map(|index| annotate_row(index, minefield))
        .collect()
}
