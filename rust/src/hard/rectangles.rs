type Dot = (char, usize, usize);
type Coord = (usize, usize);
type Rectangle = (Coord, Coord);

pub fn count(lines: &[&str]) -> u32 {
    let dots = parse_dots(lines);

    let mut rectangles = find_rectangles(&dots);
    rectangles.sort_unstable();
    rectangles.dedup();

    rectangles.len() as u32
}

fn parse_dots(lines: &[&str]) -> Vec<Dot> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, col)| col.char_indices().map(move |(x, ch)| (ch, x, y)))
        .collect()
}

pub fn find_rectangles(dots: &[Dot]) -> Vec<Rectangle> {
    let corners: Vec<Coord> = dots
        .iter()
        .filter_map(|&(ch, x, y)| if ch == '+' { Some((x, y)) } else { None })
        .collect();

    corners
        .iter()
        .flat_map(|&origin| find_dot_rectangles(origin, &corners, dots))
        .collect()
}

fn find_dot_rectangles(origin: Coord, corners: &[Coord], dots: &[Dot]) -> Vec<Rectangle> {
    let (origin_x, origin_y) = origin;

    let diagonals = corners
        .iter()
        .copied()
        .filter(|&(x, y)| x > origin_x && y > origin_y)
        .filter(|&(x, y)| corners.contains(&(origin_x, y)) && corners.contains(&(x, origin_y)));

    diagonals
        .filter(|&(diag_x, diag_y)| {
            let hor_count = dots
                .iter()
                .filter(|&&(ch, x, y)| {
                    (ch == '+' || ch == '-') &&
                    (y == origin_y || y == diag_y) &&
                    x > origin_x && x < diag_x
                })
                .count();

            let ver_count = dots
                .iter()
                .filter(|&&(ch, x, y)| {
                    (ch == '+' || ch == '|') &&
                    (x == origin_x || x == diag_x) &&
                    y > origin_y && y < diag_y
                })
                .count();

            hor_count == (diag_x - origin_x - 1) * 2 &&
            ver_count == (diag_y - origin_y - 1) * 2
        })
        .map(|diag| (origin, diag))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_rectangle() {
        #[rustfmt::skip]
        let input = &[
            "+-+",
            "| |",
            "+-+",
        ];

        let output = count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn five_rectangles_with_shared_parts() {
        #[rustfmt::skip]
        let input = &[
            "  +-+",
            "  | |",
            "+-+-+",
            "| | |",
            "+-+-+",
        ];

        let output = count(input);
        let expected = 5;
        assert_eq!(output, expected);
    }
}


