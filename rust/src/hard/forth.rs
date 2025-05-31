pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { stack: Vec::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        input
            .split_whitespace()
            .for_each(|num| self.stack.push(num.parse().unwrap()));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_just_get_pushed_onto_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 4 5").is_ok());

        assert_eq!(f.stack(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn pushes_negative_numbers_onto_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("-1 -2 -3 -4 -5").is_ok());

        assert_eq!(f.stack(), [-1, -2, -3, -4, -5]);
    }

    #[test]
    #[ignore]
    fn can_add_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 +").is_ok());

        assert_eq!(f.stack(), [3]);
    }

    #[test]
    #[ignore]
    fn errors_if_there_is_nothing_on_the_stack() {
        let mut f = Forth::new();

        assert_eq!(f.eval("+"), Err(Error::StackUnderflow));
    }

    #[test]
    #[ignore]
    fn errors_if_there_is_only_one_value_on_the_stack() {
        let mut f = Forth::new();

        assert_eq!(f.eval("1 +"), Err(Error::StackUnderflow));
    }

    #[test]
    #[ignore]
    fn more_than_two_values_on_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 +").is_ok());

        assert_eq!(f.stack(), [1, 5]);
    }
}
