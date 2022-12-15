use core::ops::Deref;

pub fn map<F, T, D>(input: Vec<T>, fun: F) -> Vec<D>
where
    F: Fn(T) -> D,
    T: Clone,
{
    map_with_acc(&input, Vec::new(), fun)
}

fn map_with_acc<F, T, D>(input: &[T], mut acc: Vec<D>, fun: F) -> Vec<D>
where
    F: Fn(T) -> D,
    T: Clone,
{
    match input {
        [] => acc,
        [num, rest @ ..] => {
            acc.push(fun((*num).clone()));
            map_with_acc(rest, acc, fun)
        }
    }
}

mod tests {
    use super::*;

    fn square(x: i32) -> i32 {
        x * x
    }

    #[test]
    fn func_single() {
        let input = vec![2];

        let expected = vec![4];

        assert_eq!(map(input, square), expected);
    }
}

#[test]
fn strings() {
    let input = vec!["1".to_string(), "2".into(), "3".into()];

    let expected = vec!["11".to_string(), "22".into(), "33".into()];

    assert_eq!(map(input, |s| s.repeat(2)), expected);
}

#[test]
fn change_in_type() {
    let input: Vec<&str> = vec!["1", "2", "3"];

    let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];

    assert_eq!(map(input, |s| s.to_string()), expected);
}

#[test]
fn minimal_bounds_on_input_and_output() {
    struct Foo;
    struct Bar;

    map(vec![Foo], |_| Bar);
}
