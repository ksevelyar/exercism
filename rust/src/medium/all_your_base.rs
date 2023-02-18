#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    validate_input(number, from_base, to_base)?;

    if is_zero(number) {
        return Ok(vec![0]);
    }

    let value10 = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (pow, digit)| {
            acc + digit * from_base.pow(pow as u32)
        });

    Ok(base10_to_base(value10, to_base, vec![]))
}

fn validate_input(number: &[u32], from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let invalid_digit = number.iter().find(|digit| **digit >= from_base);
    if let Some(digit) = invalid_digit {
        return Err(Error::InvalidDigit(*digit));
    }

    Ok(())
}

fn is_zero(number: &[u32]) -> bool {
    number.is_empty() || number.iter().all(|digit| *digit == 0)
}

fn base10_to_base(value10: u32, to_base: u32, mut digits: Vec<u32>) -> Vec<u32> {
    if value10 == 0 {
        digits.reverse();
        return digits;
    }

    let digit = value10.rem_euclid(to_base);
    let remainder = value10.div_euclid(to_base);

    digits.push(digit);

    base10_to_base(remainder, to_base, digits)
}

#[test]
fn binary_to_multiple_decimal() {
    let input_base = 2;

    let input_digits = &[1, 0, 1, 0, 1, 0];

    let output_base = 10;

    let output_digits = vec![4, 2];

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
fn decimal_to_binary() {
    let input_base = 10;

    let input_digits = &[4, 2];

    let output_base = 2;

    let output_digits = vec![1, 0, 1, 0, 1, 0];

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
fn empty_list() {
    let input_base = 2;

    let input_digits = &[];

    let output_base = 10;

    let output_digits = vec![0];

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
fn single_zero() {
    let input_base = 10;

    let input_digits = &[0];

    let output_base = 2;

    let output_digits = vec![0];

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Ok(output_digits)
    );
}

#[test]
fn invalid_positive_digit() {
    let input_base = 2;

    let input_digits = &[1, 2, 1, 0, 1, 0];

    let output_base = 10;

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidDigit(2))
    );
}

#[test]
fn input_base_is_one() {
    let input_base = 1;

    let input_digits = &[];

    let output_base = 10;

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidInputBase)
    );
}

#[test]
fn output_base_is_one() {
    let input_base = 2;

    let input_digits = &[1, 0, 1, 0, 1, 0];

    let output_base = 1;

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidOutputBase)
    );
}

#[test]
fn input_base_is_zero() {
    let input_base = 0;

    let input_digits = &[];

    let output_base = 10;

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidInputBase)
    );
}

#[test]
fn output_base_is_zero() {
    let input_base = 10;

    let input_digits = &[7];

    let output_base = 0;

    assert_eq!(
        convert(input_digits, input_base, output_base),
        Err(Error::InvalidOutputBase)
    );
}
