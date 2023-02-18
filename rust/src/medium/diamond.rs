pub fn get_diamond(c: char) -> Vec<String> {
    let range = 'A'..=c;

    let letters = range.clone().chain(range.clone().rev().skip(1));
    let length = range.count();

    let row_seq = (0..length).chain((0..length).rev().skip(1)).zip(letters);
    let col_seq = (0..length).rev().chain((0..length).skip(1));

    row_seq
        .map(|(row, letter)| {
            col_seq
                .clone()
                .map(|col| if col == row { letter } else { ' ' })
                .collect()
        })
        .collect()
}

#[test]
fn test_d() {
    #[rustfmt::skip]
    assert_eq!(
        get_diamond('D'),
        vec![
            "   A   ",
            "  B B  ",
            " C   C ",
            "D     D",
            " C   C ",
            "  B B  ",
            "   A   ",
        ]
    );
}
