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
            .to_lowercase()
            .split_whitespace()
            .try_for_each(|word| self.modify_stack(word))?;

        Ok(())
    }

    fn modify_stack(&mut self, word: &str) -> Result {
        if let Ok(num) = word.parse::<i32>() {
            self.stack.push(num);
            return Ok(());
        }

        match word {
            "+" => {
                let num1 = self.pop()?;
                let num2 = self.pop()?;

                self.stack.push(num2 + num1);
                Ok(())
            }
            "-" => {
                let num1 = self.pop()?;
                let num2 = self.pop()?;

                self.stack.push(num2 - num1);
                Ok(())
            }
            "*" => {
                let num1 = self.pop()?;
                let num2 = self.pop()?;

                self.stack.push(num2 * num1);
                Ok(())
            }
            "/" => {
                let num1 = self.pop()?;
                if num1 == 0 {
                    return Err(Error::DivisionByZero);
                }

                let num2 = self.pop()?;

                self.stack.push(num2 / num1);
                Ok(())
            }
            "drop" => {
                self.pop()?;
                Ok(())
            }
            "swap" => {
                let num1 = self.pop()?;
                let num2 = self.pop()?;

                self.stack.push(num1);
                self.stack.push(num2);
                Ok(())
            }
            "dup" => {
                let last = self.stack.last().ok_or(Error::StackUnderflow)?;

                self.stack.push(*last);
                Ok(())
            }
            "over" => {
                if self.stack.len() >= 2 {
                    let num = &self.stack[self.stack.len() - 2];

                    self.stack.push(*num);
                    Ok(())
                } else {
                    Err(Error::StackUnderflow)
                }
            }
            _ => Err(Error::UnknownWord),
        }
    }

    fn pop(&mut self) -> std::result::Result<i32, Error> {
        match self.stack.pop() {
            Some(num) => Ok(num),
            _ => Err(Error::StackUnderflow),
        }
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
    fn can_add_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 +").is_ok());

        assert_eq!(f.stack(), [3]);
    }

    #[test]
    fn errors_if_there_is_nothing_on_the_stack() {
        let mut f = Forth::new();

        assert_eq!(f.eval("+"), Err(Error::StackUnderflow));
    }

    #[test]
    fn errors_if_there_is_only_one_value_on_the_stack() {
        let mut f = Forth::new();

        assert_eq!(f.eval("1 +"), Err(Error::StackUnderflow));
    }

    #[test]
    fn more_than_two_values_on_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 +").is_ok());

        assert_eq!(f.stack(), [1, 5]);
    }

    #[test]
    fn can_subtract_two_numbers() {
        let mut f = Forth::new();

        assert!(f.eval("3 4 -").is_ok());

        assert_eq!(f.stack(), [-1]);
    }

    #[test]
    fn performs_integer_division() {
        let mut f = Forth::new();

        assert!(f.eval("8 3 /").is_ok());

        assert_eq!(f.stack(), [2]);
    }

    #[test]
    fn errors_if_dividing_by_zero() {
        let mut f = Forth::new();

        assert_eq!(f.eval("4 0 /"), Err(Error::DivisionByZero));
    }

    #[test]
    fn copies_the_top_value_on_the_stack() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 dup").is_ok());

        assert_eq!(f.stack(), [1, 2, 2]);
    }

    #[test]
    fn removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 drop").is_ok());

        assert_eq!(f.stack(), [1]);
    }

    #[test]
    fn swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 swap").is_ok());

        assert_eq!(f.stack(), [1, 3, 2]);
    }

    #[test]
    fn copies_the_second_element_if_there_are_more_than_two() {
        let mut f = Forth::new();

        assert!(f.eval("1 2 3 over").is_ok());

        assert_eq!(f.stack(), [1, 2, 3, 2]);
    }

    #[test]
    #[ignore]
    fn can_consist_of_built_in_words() {
        let mut f = Forth::new();

        assert!(f.eval(": dup-twice dup dup ;").is_ok());

        assert!(f.eval("1 dup-twice").is_ok());

        assert_eq!(f.stack(), [1, 1, 1]);
    }

    #[test]
    #[ignore]
    fn execute_in_the_right_order() {
        let mut f = Forth::new();

        assert!(f.eval(": countup 1 2 3 ;").is_ok());

        assert!(f.eval("countup").is_ok());

        assert_eq!(f.stack(), [1, 2, 3]);
    }

    #[test]
    #[ignore]
    fn can_override_other_user_defined_words() {
        let mut f = Forth::new();

        assert!(f.eval(": foo dup ;").is_ok());

        assert!(f.eval(": foo dup dup ;").is_ok());

        assert!(f.eval("1 foo").is_ok());

        assert_eq!(f.stack(), [1, 1, 1]);
    }

    #[test]
    #[ignore]
    fn can_override_built_in_words() {
        let mut f = Forth::new();

        assert!(f.eval(": swap dup ;").is_ok());

        assert!(f.eval("1 swap").is_ok());

        assert_eq!(f.stack(), [1, 1]);
    }

    #[test]
    #[ignore]
    fn can_override_built_in_operators() {
        let mut f = Forth::new();

        assert!(f.eval(": + * ;").is_ok());

        assert!(f.eval("3 4 +").is_ok());

        assert_eq!(f.stack(), [12]);
    }

    #[test]
    #[ignore]
    fn can_use_different_words_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 5 ;").is_ok());

        assert!(f.eval(": bar foo ;").is_ok());

        assert!(f.eval(": foo 6 ;").is_ok());

        assert!(f.eval("bar foo").is_ok());

        assert_eq!(f.stack(), [5, 6]);
    }

    #[test]
    #[ignore]
    fn can_define_word_that_uses_word_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 10 ;").is_ok());

        assert!(f.eval(": foo foo 1 + ;").is_ok());

        assert!(f.eval("foo").is_ok());

        assert_eq!(f.stack(), [11]);
    }

    #[test]
    #[ignore]
    fn cannot_redefine_non_negative_numbers() {
        let mut f = Forth::new();

        assert_eq!(f.eval(": 1 2 ;"), Err(Error::InvalidWord));
    }

    #[test]
    #[ignore]
    fn cannot_redefine_negative_numbers() {
        let mut f = Forth::new();

        assert_eq!(f.eval(": -1 2 ;"), Err(Error::InvalidWord));
    }

    #[test]
    fn errors_if_executing_a_non_existent_word() {
        let mut f = Forth::new();

        assert_eq!(f.eval("foo"), Err(Error::UnknownWord));
    }

    #[test]
    #[ignore]
    fn only_defines_locally() {
        let mut f = Forth::new();

        assert!(f.eval(": + - ;").is_ok());

        assert!(f.eval("1 1 +").is_ok());

        assert_eq!(f.stack(), [0]);

        let mut f = Forth::new();

        assert!(f.eval("1 1 +").is_ok());

        assert_eq!(f.stack(), [2]);
    }
}
