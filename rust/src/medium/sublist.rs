#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let is_equal_bool = first_list
        .iter()
        .zip(second_list)
        .all(|(first_list_item, second_list_item)| first_list_item == second_list_item);

    match is_equal_bool {
        true => Comparison::Equal,
        false => Comparison::Unequal,
    }
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
