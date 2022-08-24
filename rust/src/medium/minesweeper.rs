use std::str;

fn annotate_first_row(minefield: &[&str]) -> String {
    "1".to_owned()
}

fn annotate_last_row(minefield: &[&str]) -> String {
    "last".to_owned()
}

fn annotate_triplet_row(row_index: usize, minefield: &[&str]) -> String {
    "triplet".to_owned()
}

fn annotate_row(row_index: usize, minefield: &[&str]) -> String {
    let last_row = minefield.len() - 1;

    match row_index {
        0 => annotate_first_row(minefield),
        _ if row_index == last_row => annotate_last_row(minefield),
        _ => annotate_triplet_row(row_index, minefield),
    }
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
