pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let matrix: Vec<Vec<_>> = (0..size)
        .map(|row| (0..size).map(|column| (row, column)).collect())
        .collect();

    dbg!(matrix);

    // (0..(size * size)).fold(matrix, |matrix, item| matrix)
    todo!()
}

#[test]
fn size_three_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1, 2, 3],
        vec![8, 9, 4],
        vec![7, 6, 5]
    ];

    assert_eq!(spiral_matrix(3), expected);
}

#[test]
fn size_four_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1,  2,  3,  4],
        vec![12, 13, 14, 5],
        vec![11, 16, 15, 6],
        vec![10, 9,  8,  7],
    ];

    assert_eq!(spiral_matrix(4), expected);
}

#[test]
fn size_five_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1,  2,  3,  4,  5],
        vec![16, 17, 18, 19, 6],
        vec![15, 24, 25, 20, 7],
        vec![14, 23, 22, 21, 8],
        vec![13, 12, 11, 10, 9],
    ];

    assert_eq!(spiral_matrix(5), expected);
}
