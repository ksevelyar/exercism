use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..(sum / 3))
        .flat_map(|a| {
            ((a + 1)..(sum / 2))
                .take_while(move |b| a + b < sum)
                .map(move |b| [a, b, sum - a - b])
                .filter(|[a, b, c]| a.pow(2) + b.pow(2) == c.pow(2))
        })
        .collect()
}

#[test]
fn test_triplets_whose_sum_is_12() {
    let expected: HashSet<_> = [[3, 4, 5]].iter().cloned().collect();
    assert_eq!(find(12), expected);
}

#[test]
fn test_triplets_whose_sum_is_1000() {
    let expected: HashSet<_> = [[200, 375, 425]].iter().cloned().collect();
    assert_eq!(find(1000), expected);
}

#[test]
fn test_triplets_for_large_number() {
    let expected: HashSet<_> = [
        [1200, 14_375, 14_425],
        [1875, 14_000, 14_125],
        [5000, 12_000, 13_000],
        [6000, 11_250, 12_750],
        [7500, 10_000, 12_500],
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(find(30_000), expected);
}
