use std::str;

fn annotate_item(index: usize, item: u8, neighbours: &Vec<&[u8]>) -> String {
    let mine: u8 = 42;
    match item {
        _ if item == mine => "*".to_owned(),
        _ => "x".to_owned(),
    }
}

fn neighbours<'a>(index: usize, minefield: &'a [&str]) -> Vec<&'a [u8]> {
    let prev_row = index.saturating_sub(1);
    let next_row = match index {
        _ if index == minefield.len() - 1 => index,
        _ => index + 1,
    };

    let rows = &minefield[prev_row..=next_row];
    dbg!(rows).iter().map(|row| row.as_bytes()).collect()
}

fn annotate_row(index: usize, minefield: &[&str]) -> String {
    let row = minefield[index].as_bytes();
    let neighbours = neighbours(index, minefield);

    row.iter()
        .enumerate()
        .map(|(index, item)| annotate_item(index, *item, &neighbours))
        .collect()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    dbg!(minefield[0].as_bytes());

    (0..minefield.len())
        .map(|index| annotate_row(index, minefield))
        .collect()
}
