pub fn find(array: &[i32], key: i32) -> Option<usize> {
    find_with_acc(array, key, 0)
}

fn find_with_acc(array: &[i32], key: i32, index: usize) -> Option<usize> {
    let mid_index = array.len() / 2;
    let mid = *array.get(mid_index)?;

    match mid {
        _ if mid < key => find_with_acc(&array[mid_index..], key, index + mid_index),
        _ if mid > key => find_with_acc(&array[..mid_index], key, 0),
        _ => Some(index + mid_index),
    }
}

#[test]
fn finds_a_value_in_an_array_with_one_element() {
    assert_eq!(find(&[6], 6), Some(0));
}

#[test]
fn finds_a_value_in_an_array_of_odd_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
}

#[test]
fn finds_a_value_in_an_array_of_even_length() {
    assert_eq!(
        find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
}
