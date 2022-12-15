pub fn map<F, T, D>(input: Vec<T>, fun: F) -> Vec<D>
where
    F: FnMut(T) -> D,
{
    map_with_acc(input, Vec::new(), fun)
}

fn map_with_acc<F, T, D>(mut input: Vec<T>, mut acc: Vec<D>, mut fun: F) -> Vec<D>
where
    F: FnMut(T) -> D,
{
    if input.is_empty() {
        acc.reverse();
        return acc;
    }

    acc.push(fun(input.pop().unwrap()));
    map_with_acc(input, acc, fun)
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

#[test]
fn mutating_closure() {
    let mut counter = 0;

    let input = vec![-2, 3, 4, -5];

    let expected = vec![2, 3, 4, 5];

    let result = map(input, |x: i64| {
        counter += 1;

        x.abs()
    });

    assert_eq!(result, expected);

    assert_eq!(counter, 4);
}
