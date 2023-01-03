fn is_biggest_in_row(x: u64, row: &[u64]) -> bool {
    *row.iter().max().unwrap_or(&0) == x
}
fn is_smallest_in_column(x: u64, column: &[u64]) -> bool {
    *column.iter().min().unwrap_or(&0) == x
}

pub fn find_saddle_points(rows: &[Vec<u64>]) -> Vec<(usize, usize)> {
    rows.iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(move |(column_index, x)| {
                    let column: Vec<u64> = rows
                        .iter()
                        .filter_map(|row| row.get(*column_index))
                        .cloned()
                        .collect();

                    is_biggest_in_row(**x, row) && is_smallest_in_column(**x, &column)
                })
                .map(move |(column_index, _)| (row_index, column_index))
        })
        .collect()
}

#[test]
fn identify_single_saddle_point() {
    let input = vec![vec![9, 8, 7], vec![5, 3, 2], vec![6, 6, 7]];

    assert_eq!(vec![(1, 0)], find_saddle_points(&input));
}
