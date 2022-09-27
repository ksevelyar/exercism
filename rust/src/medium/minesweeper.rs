use std::str;

fn annotate_item(index: usize, item: u8, neighbours: [Option<&[u8]>; 2]) -> String {
    let mine: u8 = 42;
    match item {
      _ if item == mine => "*".to_owned(),
      _ => "x".to_owned()
    }
}

fn prev_row<'a>(index: usize, minefield: &'a [&str]) -> Option<&'a [u8]> {
    match index {
        0 => None,
        _ => Some(minefield[index - 1].as_bytes()),
    }
}
fn next_row<'a>(index: usize, minefield: &'a [&str]) -> Option<&'a [u8]> {
    Some(minefield.get(index + 1)?.as_bytes())
}

fn annotate_row(index: usize, minefield: &[&str]) -> String {
    let row = minefield[index].as_bytes();
    let neighbours = [prev_row(index, minefield), next_row(index, minefield)];

    row.iter()
        .enumerate()
        .map(|(index, item)| annotate_item(index, *item, neighbours))
        .collect()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    dbg!(minefield[0].as_bytes());

    (0..minefield.len())
        .map(|index| annotate_row(index, minefield))
        .collect()
}
