pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    fn row(max_ind: u32, prev_row: Option<&Vec<u32>>) -> Vec<u32> {
        (0..max_ind)
            .map(|x| {
                if x == 0 || x == max_ind {
                    return 1;
                }

                let prev_row = prev_row.expect("exists for x > 0");
                prev_row[(x - 1) as usize] + prev_row.get(x as usize).unwrap_or(&0)
            })
            .collect()
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.row_count).fold(Vec::new(), |mut acc, row_ind| {
            acc.push(Self::row(row_ind, acc.last()));
            acc
        })
    }
}

#[test]
fn two_rows() {
    let pt = PascalsTriangle::new(2);

    let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];

    assert_eq!(expected, pt.rows());
}

#[test]
fn ten_rows() {
    let pt = PascalsTriangle::new(10);

    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
        vec![1, 7, 21, 35, 35, 21, 7, 1],
        vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
        vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    ];

    assert_eq!(expected, pt.rows());
}
