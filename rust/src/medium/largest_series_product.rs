#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if span == 0 {
        return Ok(1);
    }

    let digits: Result<Vec<_>, Error> = string_digits
        .chars()
        .map(|x| match x.to_digit(10) {
            Some(digit) => Ok(digit as u64),
            None => Err(Error::InvalidDigit(x)),
        })
        .collect();

    Ok(digits?
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .unwrap())
}

#[test]
fn find_the_largest_product_of_two_with_numbers_in_order() {
    assert_eq!(Ok(72), lsp("0123456789", 2));
}

#[test]
fn a_span_is_longer_than_number_is_an_error() {
    assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
}

#[test]
fn an_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("", 0));
}

#[test]
fn a_non_empty_string_and_no_span_returns_one() {
    assert_eq!(Ok(1), lsp("123", 0));
}

#[test]
fn a_string_with_non_digits_is_an_error() {
    assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
}
