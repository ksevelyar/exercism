pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let (first, unlinked) = input.split_at(1);

    build_chain(first.to_vec(), unlinked.to_vec())
}

fn build_chain(chain: Vec<(u8, u8)>, unlinked: Vec<(u8, u8)>) -> Option<Vec<(u8, u8)>> {
    let (first, _) = *chain.first()?;
    let (_, last) = *chain.last()?;

    if unlinked.is_empty() && first == last {
        return Some(chain);
    }

    unlinked.iter().enumerate().find_map(|(i, &domino)| {
        maybe_rotate_domino(domino, last).and_then(|rotated| {
            let mut new_chain = chain.clone();
            new_chain.push(rotated);

            let mut new_unlinked = unlinked.clone();
            new_unlinked.remove(i);

            build_chain(new_chain, new_unlinked)
        })
    })
}

fn maybe_rotate_domino(domino: (u8, u8), target: u8) -> Option<(u8, u8)> {
    match domino {
        (x, y) if x == target => Some((x, y)),
        (x, y) if y == target => Some((y, x)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_empty_output() {
        let input = &[];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn singleton_input_singleton_output() {
        let input = &[(1, 1)];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn singleton_that_can_t_be_chained() {
        let input = &[(1, 2)];

        assert!(chain(input).is_none());
    }

    #[test]
    fn three_elements() {
        let input = &[(1, 2), (3, 1), (2, 3)];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn can_reverse_dominoes() {
        let input = &[(1, 2), (1, 3), (2, 3)];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn can_t_be_chained() {
        let input = &[(1, 2), (4, 1), (2, 3)];

        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_simple() {
        let input = &[(1, 1), (2, 2)];

        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_double_loop() {
        let input = &[(1, 2), (2, 1), (3, 4), (4, 3)];

        assert!(chain(input).is_none());
    }

    #[test]
    fn disconnected_single_isolated() {
        let input = &[(1, 2), (2, 3), (3, 1), (4, 4)];

        assert!(chain(input).is_none());
    }

    #[test]
    fn need_backtrack() {
        let input = &[(1, 2), (2, 3), (3, 1), (2, 4), (2, 4)];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn separate_loops() {
        let input = &[(1, 2), (2, 3), (3, 1), (1, 1), (2, 2), (3, 3)];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn nine_elements() {
        let input = &[
            (1, 2),
            (5, 3),
            (3, 1),
            (1, 2),
            (2, 4),
            (1, 6),
            (2, 3),
            (3, 4),
            (5, 6),
        ];

        let output = chain(input);

        assert!(output.is_some());

        assert_correct(input, output.unwrap());
    }

    #[test]
    fn separate_three_domino_loops() {
        let input = &[(1, 2), (2, 3), (3, 1), (4, 5), (5, 6), (6, 4)];

        assert!(chain(input).is_none());
    }

    type Domino = (u8, u8);

    fn assert_correct(input: &[Domino], output: Vec<Domino>) {
        if input.len() != output.len() {
            panic!("Length mismatch for input {input:?}, output {output:?}");
        } else if input.is_empty() {
            // and thus output.is_empty()
            return;
        }

        let mut output_sorted = output
            .iter()
            .map(|&d| normalize(d))
            .collect::<Vec<Domino>>();

        output_sorted.sort_unstable();

        let mut input_sorted = input.iter().map(|&d| normalize(d)).collect::<Vec<Domino>>();

        input_sorted.sort_unstable();

        if input_sorted != output_sorted {
            panic!("Domino mismatch for input {input:?}, output {output:?}");
        }

        // both input and output have at least 1 element
        // This essentially puts the first element after the last one, thereby making it
        // easy to check whether the domino chains "wraps around".
        {
            let mut n = output[0].1;

            let iter = output.iter().skip(1).chain(output.iter().take(1));

            for &(first, second) in iter {
                if n != first {
                    panic!("Chaining failure for input {input:?}, output {output:?}")
                }

                n = second
            }
        }
    }

    fn normalize(d: Domino) -> Domino {
        match d {
            (m, n) if m > n => (n, m),

            (m, n) => (m, n),
        }
    }
}
