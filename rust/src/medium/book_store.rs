pub fn lowest_price(books: &[u32]) -> u32 {
    books.iter().map(|_book| 800).sum()
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
