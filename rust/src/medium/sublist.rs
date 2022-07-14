#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    Comparison::Equal
}

fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    Comparison::Superlist
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    Comparison::Sublist
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (first, last) if first > last => is_superlist(first_list, second_list),
        (first, last) if first < last => is_sublist(first_list, second_list),
        _ => is_equal(first_list, second_list),
    }
}
