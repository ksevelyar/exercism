pub struct Forth {
    stack: Vec<i32>,
    definitions: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            definitions: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[i32] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        if input.starts_with(": ") && input.ends_with(" ;") {
            let inner = &input[2..input.len() - 2];
            let mut parts = inner.split_whitespace();

            let key = parts.next().ok_or(Error::InvalidWord)?;

            if key.parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }

            self.definitions.push(input.to_lowercase());
            return Ok(());
        }

        input
            .to_lowercase()
            .split_whitespace()
            .try_for_each(|word| {
                self.eval_word(word, 0)
                    .iter()
                    .try_for_each(|word| self.modify_stack(word))?;
                Ok(())
            })?;

        Ok(())
    }

    fn modify_stack(&mut self, word: &str) -> Result<(), Error> {
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
                let divisor = self.pop()?;
                if divisor == 0 {
                    return Err(Error::DivisionByZero);
                }
                let dividend = self.pop()?;

                self.stack.push(dividend / divisor);
                Ok(())
            }
            "drop" => {
                self.pop()?;
                Ok(())
            }
            "swap" => {
                let len = self.stack.len();
                if len < 2 {
                    return Err(Error::StackUnderflow);
                }

                self.stack.swap(len - 2, len - 1);
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

    fn eval_word(&mut self, word: &str, shift: usize) -> Vec<String> {
        let result = self
            .definitions
            .iter()
            .enumerate()
            .rev()
            .skip(shift)
            .find_map(|(index, definition)| {
                let inner = &definition[2..definition.len() - 2];
                let mut parts = inner.split_whitespace();

                let definition_key = parts.next().unwrap();

                if definition_key == word {
                    Some((index, parts.map(|x| x.to_string()).collect::<Vec<_>>()))
                } else {
                    None
                }
            });

        if result.is_none() {
            return vec![word.to_string()];
        }

        let (new_shift, values) = result.unwrap();
        values
            .iter()
            .flat_map(|word| self.eval_word(word, shift + new_shift))
            .collect()
    }

    fn pop(&mut self) -> Result<i32, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
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
    fn can_consist_of_built_in_words() {
        let mut f = Forth::new();

        assert!(f.eval(": dup-twice dup dup ;").is_ok());

        assert!(f.eval("1 dup-twice").is_ok());

        assert_eq!(f.stack(), [1, 1, 1]);
    }

    #[test]
    fn execute_in_the_right_order() {
        let mut f = Forth::new();

        assert!(f.eval(": countup 1 2 3 ;").is_ok());

        assert!(f.eval("countup").is_ok());

        assert_eq!(f.stack(), [1, 2, 3]);
    }

    #[test]
    fn can_override_other_user_defined_words() {
        let mut f = Forth::new();

        assert!(f.eval(": foo dup ;").is_ok());

        assert!(f.eval(": foo dup dup ;").is_ok());

        assert!(f.eval("1 foo").is_ok());

        assert_eq!(f.stack(), [1, 1, 1]);
    }

    #[test]
    fn can_override_built_in_words() {
        let mut f = Forth::new();

        assert!(f.eval(": swap dup ;").is_ok());

        assert!(f.eval("1 swap").is_ok());

        assert_eq!(f.stack(), [1, 1]);
    }

    #[test]
    fn can_override_built_in_operators() {
        let mut f = Forth::new();

        assert!(f.eval(": + * ;").is_ok());

        assert!(f.eval("3 4 +").is_ok());

        assert_eq!(f.stack(), [12]);
    }

    #[test]
    fn can_use_different_words_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 5 ;").is_ok());

        assert!(f.eval(": bar foo ;").is_ok());

        assert!(f.eval(": foo 6 ;").is_ok());

        assert!(f.eval("bar foo").is_ok());

        assert_eq!(f.stack(), [5, 6]);
    }

    #[test]
    fn can_define_word_that_uses_word_with_the_same_name() {
        let mut f = Forth::new();

        assert!(f.eval(": foo 10 ;").is_ok());

        assert!(f.eval(": foo foo 1 + ;").is_ok());

        assert!(f.eval("foo").is_ok());

        assert_eq!(f.stack(), [11]);
    }

    #[test]
    fn cannot_redefine_non_negative_numbers() {
        let mut f = Forth::new();

        assert_eq!(f.eval(": 1 2 ;"), Err(Error::InvalidWord));
    }

    #[test]
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
    fn only_defines_locally() {
        let mut f = Forth::new();

        assert!(f.eval(": + - ;").is_ok());

        assert!(f.eval("1 1 +").is_ok());

        assert_eq!(f.stack(), [0]);

        let mut f = Forth::new();

        assert!(f.eval("1 1 +").is_ok());

        assert_eq!(f.stack(), [2]);
    }

    #[test]
    fn definitions_are_case_insensitive() {
        let mut f = Forth::new();

        assert!(f.eval(": SWAP DUP Dup dup ;").is_ok());

        assert!(f.eval("1 swap").is_ok());

        assert_eq!(f.stack(), [1, 1, 1, 1]);
    }
}
