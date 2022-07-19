#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_equal<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list
        .iter()
        .zip(second_list)
        .all(|(first_list_item, second_list_item)| first_list_item == second_list_item)
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let iterations = first_list.len() - second_list.len() + 1;

    first_list
        .iter()
        .take(iterations)
        .enumerate()
        .any(|(index, item)| {
            item == &second_list[0] && is_equal(&first_list[index..=second_list.len()], second_list)
        })
}

fn equality_comparison<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match is_equal(first_list, second_list) {
        true => Comparison::Equal,
        false => Comparison::Unequal,
    }
}

fn superlist_comparison<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if second_list.len() == 0 {
        return Comparison::Superlist;
    }

    match is_sublist(first_list, second_list) {
        true => Comparison::Superlist,
        false => Comparison::Unequal,
    }
}

fn sublist_comparison<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == 0 {
        return Comparison::Sublist;
    }

    match is_sublist(second_list, first_list) {
        true => Comparison::Sublist,
        false => Comparison::Unequal,
    }
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let a_len = first_list.len();
    let b_len = second_list.len();

    match a_len > b_len {
        true => superlist_comparison(first_list, second_list),
        false if a_len != b_len => sublist_comparison(first_list, second_list),
        false => equality_comparison(first_list, second_list),
    }
}
