const REGULAR_PRICE: u32 = 800;
const BATCH_LIMIT: usize = 5;

fn batch_price(num: usize) -> u32 {
    let discount = match num {
        2 => REGULAR_PRICE / 100 * 5,
        3 => REGULAR_PRICE / 100 * 10,
        4 => REGULAR_PRICE / 100 * 20,
        5 => REGULAR_PRICE / 100 * 25,
        _ => 0,
    };

    (REGULAR_PRICE - discount) * num as u32
}

fn possible_sums(books: &[u32], limit: usize) -> u32 {
    let batch = books
        .iter()
        .cloned()
        .fold(Vec::new(), |mut sets: Vec<Vec<u32>>, book| {
            let set = sets
                .iter_mut()
                .find(|set| !set.contains(&book) && set.len() < limit);

            match set {
                None => sets.push(vec![book]),
                Some(set) => set.push(book),
            };

            sets
        });

    batch.iter().map(|batch| batch_price(batch.len())).sum()
}

pub fn lowest_price(books: &[u32]) -> u32 {
    (2..=BATCH_LIMIT)
        .map(|limit| possible_sums(books, limit))
        .min()
        .unwrap_or(0)
}

#[test]
fn test_only_a_single_book() {
    assert_eq!(lowest_price(&vec![1]), 800);
}

#[test]
fn test_two_of_the_same_book() {
    assert_eq!(lowest_price(&vec![2, 2]), 1_600);
}

#[test]
fn test_two_different_books() {
    assert_eq!(lowest_price(&vec![1, 2]), 1_520);
}

#[test]
fn test_three_different_books() {
    assert_eq!(lowest_price(&vec![1, 2, 3]), 2_160);
}

#[test]
fn test_four_different_books() {
    assert_eq!(lowest_price(&vec![1, 2, 3, 4]), 2_560);
}

#[test]
fn test_five_different_books() {
    assert_eq!(lowest_price(&vec![1, 2, 3, 4, 5]), 3_000);
}

#[test]
fn test_group_of_four_plus_group_of_two_is_cheaper_than_two_groups_of_three() {
    assert_eq!(lowest_price(&vec![1, 1, 2, 2, 3, 4]), 4_080);
}

#[test]
fn test_two_groups_of_four_is_cheaper_than_group_of_five_plus_group_of_three() {
    assert_eq!((lowest_price(&vec![1, 1, 2, 2, 3, 3, 4, 5])), 5_120,);
}
