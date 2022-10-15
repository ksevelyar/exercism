const ISBN_LENGTH: u32 = 10;

fn parse_isbn_digit((index, char): (usize, &char)) -> Option<u32> {
    match (index, char) {
        (9, 'X') => Some(10),
        (_, char) => char.to_digit(10),
    }
}

fn is_valid_isbn_numbers(numbers: &[u32]) -> bool {
    let isbn_sum: u32 = numbers
        .iter()
        .enumerate()
        .map(|(index, num)| *num * (ISBN_LENGTH - index as u32))
        .sum();

    isbn_sum % 11 == 0
}

pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars: Vec<char> = isbn.chars().filter(|char| *char != '-').collect();
    if chars.len() as u32 != ISBN_LENGTH {
        return false;
    };

    let digits: Option<Vec<u32>> = chars.iter().enumerate().map(parse_isbn_digit).collect();
    match digits {
        Some(vec) => is_valid_isbn_numbers(&vec),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_isbn() {
        assert_eq!(is_valid_isbn("3-598-21508-8"), true);
    }
}
