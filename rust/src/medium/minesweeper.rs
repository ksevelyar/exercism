fn mines_around(index: usize, neighbours: &Vec<&[u8]>) -> usize {
    let mine: u8 = 42;
    let prev_column = index.saturating_sub(1);
    let next_column = match index {
        _ if index == neighbours[0].len() - 1 => index,
        _ => index + 1,
    };

    neighbours
        .iter()
        .map(|row| &row[prev_column..=next_column])
        .flatten()
        .filter(|item| **item == mine)
        .count()
}

fn annotate_item(index: usize, item: u8, neighbours: &Vec<&[u8]>) -> String {
    let mine: u8 = 42;

    if item == mine {
        return "*".to_owned();
    }

    match mines_around(index, neighbours) {
        0 => " ".to_owned(),
        count => count.to_string(),
    }
}

fn neighbours<'a>(index: usize, minefield: &'a [&str]) -> Vec<&'a [u8]> {
    let prev_row = index.saturating_sub(1);
    let next_row = match index {
        _ if index == minefield.len() - 1 => index,
        _ => index + 1,
    };

    minefield[prev_row..=next_row]
        .iter()
        .map(|row| row.as_bytes())
        .collect()
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
    (0..minefield.len())
        .map(|index| annotate_row(index, minefield))
        .collect()
}
