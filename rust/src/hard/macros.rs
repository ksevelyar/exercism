#[macro_export]
macro_rules! hashmap {
    ( $($key:tt => $val:expr $(,)?),* ) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert($key, $val);
        )*
        hm
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let expected: HashMap<u32, u32> = HashMap::new();

        let computed: HashMap<u32, u32> = hashmap!();

        assert_eq!(computed, expected);
    }

    #[test]
    fn single() {
        let mut expected = HashMap::new();

        expected.insert(1, "one");

        assert_eq!(hashmap!(1 => "one"), expected);
    }

    #[test]
    fn no_trailing_comma() {
        let mut expected = HashMap::new();

        expected.insert(1, "one");

        expected.insert(2, "two");

        assert_eq!(hashmap!(1 => "one", 2 => "two"), expected);
    }

    #[test]
    fn trailing_comma() {
        let mut expected = HashMap::new();

        expected.insert('h', 89);

        expected.insert('a', 1);

        expected.insert('s', 19);

        expected.insert('h', 8);

        assert_eq!(
            hashmap!(
                'h' => 89,
                'a' => 1,
                's' => 19,
                'h' => 8,
            ),
            expected
        );
    }
}
