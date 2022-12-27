const REGULAR_PRICE: u32 = 800;

fn batch_price(num: usize) -> u32 {
    let discount = match num {
        2 => REGULAR_PRICE / 100 * 5,
        3 => REGULAR_PRICE / 100 * 10,
        4 => REGULAR_PRICE / 100 * 20,
        5 => REGULAR_PRICE / 100 * 25,
        _ => 0
    };

    (REGULAR_PRICE - discount) * num as u32
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let batch = books.iter().cloned().fold(Vec::new(), |mut acc, book| {
        if !acc.contains(&book) {
            acc.push(book);
        }

        acc
    });

    batch_price(batch.len())
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
