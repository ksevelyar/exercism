//! <https://exercism.org/tracks/rust/exercises/book-store>

pub fn lowest_price(books: &[u32]) -> u32 {
    books.iter().map(|_book| 800).sum()
}

#[test]
fn test_only_a_single_book() {
    assert_eq!(lowest_price(&vec![1]), 800);
}
