const ROW_LEN: usize = 4;
const COL_LEN: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<_> = input.lines().collect();

    if lines.len() % ROW_LEN != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    if lines.iter().any(|line| line.len() % COL_LEN != 0) {
        return Err(Error::InvalidColumnCount(lines.len()));
    }

    let rows: Vec<_> = lines.chunks(ROW_LEN).map(parse_line).collect();

    Ok(rows.join(","))
}

fn parse_line(rows: &[&str]) -> String {
    let digits: Vec<_> = rows
        .iter()
        .map(|row| {
            row.as_bytes()
                .chunks(COL_LEN)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (0..digits[0].len())
        .map(|i| digits.iter().map(|row| row[i]).collect::<Vec<_>>())
        .map(|digit| parse_digit(&digit))
        .collect()
}

#[rustfmt::skip]
fn parse_digit(digit: &[&str]) -> &'static str {
    match digit {
        [
           "   ",
           "  |",
           "  |",
           "   "
        ] => "1",
        [
            " _ ",
            " _|",
            "|_ ",
            "   ",
        ] => "2",
        [
            " _ ",
            " _|",
            " _|",
            "   ",
        ] => "3",
        [
            "   ",
            "|_|",
            "  |",
            "   ",
        ] => "4",
        [
            " _ ",
            "|_ ",
            " _|",
            "   ",
        ] => "5",
        [
            " _ ",
            "|_ ",
            "|_|",
            "   ",
        ] => "6",
        [
            " _ ",
            "  |",
            "  |",
            "   ",
        ] => "7",
        [
            " _ ",
            "|_|",
            "|_|",
            "   ",
        ] => "8",
        [
            " _ ",
            "|_|",
            " _|",
            "   ",
        ] => "9",
        [
            " _ ",
            "| |",
            "|_|",
            "   ",
        ] => "0",
        _ => "?",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_0() {
        #[rustfmt::skip]
        let input = " _ \n".to_string() +
                    "| |\n" +
                    "|_|\n" +
                    "   ";

        assert_eq!(Ok("0".to_string()), convert(&input));
    }

    #[test]
    fn recognizes_string_of_decimal_numbers() {
        #[rustfmt::skip]
        let input =
        "    _  _     _  _  _  _  _  _ \n".to_string() +
        "  | _| _||_||_ |_   ||_||_|| |\n" +
        "  ||_  _|  | _||_|  ||_| _||_|\n" +
        "                              ";
        assert_eq!(Ok("1234567890".to_string()), convert(&input));
    }

    #[test]
    fn numbers_across_multiple_lines_are_joined_by_commas() {
        #[rustfmt::skip]
        let input = "    _  _ \n".to_string() +
                    "  | _| _|\n" +
                    "  ||_  _|\n" +
                    "         \n" +
                    "    _  _ \n" +
                    "|_||_ |_ \n" +
                    "  | _||_|\n" +
                    "         \n" +
                    " _  _  _ \n" +
                    "  ||_||_|\n" +
                    "  ||_| _|\n" +
                    "         ";

        assert_eq!(Ok("123,456,789".to_string()), convert(&input));
    }
}
