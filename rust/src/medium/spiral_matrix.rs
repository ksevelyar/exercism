#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug)]
struct MatrixCrawler {
    direction: Direction,
    xy: (u32, u32),
    size: u32,
    moves: Vec<(u32, u32)>,
}

impl MatrixCrawler {
    fn new(size: u32) -> MatrixCrawler {
        MatrixCrawler {
            direction: Direction::Right,
            xy: (0, 0),
            size,
            moves: vec![(0, 0)],
        }
    }
}

impl Iterator for MatrixCrawler {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.moves.len() > (self.size * self.size) as usize {
            return None;
        }

        let current = self.xy;
        let (x, y) = self.xy;
        if is_direction_blocked(self.direction, x, y, self.size) {
            self.direction = turn_clockwise(self.direction);
        };

        let maybe_new_position = next_xy(self.direction, x, y);
        let is_way_blocked = self.moves.contains(&maybe_new_position);

        let new_xy = if is_way_blocked {
            self.direction = turn_clockwise(self.direction);
            next_xy(self.direction, x, y)
        } else {
            maybe_new_position
        };

        self.moves.push(new_xy);
        self.xy = new_xy;
        Some(current)
    }
}

fn next_xy(direction: Direction, x: u32, y: u32) -> (u32, u32) {
    use Direction::*;
    match direction {
        Right => (x + 1, y),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Up => (x, y - 1),
    }
}

fn is_direction_blocked(direction: Direction, x: u32, y: u32, size: u32) -> bool {
    use Direction::*;
    match direction {
        Right => x + 1 == size,
        Down => y + 1 == size,
        Left => x == 0,
        Up => y == 0,
    }
}

fn turn_clockwise(direction: Direction) -> Direction {
    use Direction::*;
    match direction {
        Right => Down,
        Down => Left,
        Left => Up,
        Up => Right,
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let moves = MatrixCrawler::new(size).collect::<Vec<_>>();
    if moves.is_empty() {
        return vec![];
    }

    let mut grid = moves.clone();
    grid.sort_unstable_by_key(|x| (x.1, x.0));
    grid.chunks(size as usize)
        .map(|row| {
            row.iter()
                .map(|xy| {
                    moves
                        .iter()
                        .position(|item| item == xy)
                        .map(|item| item + 1)
                        .expect("the grid always maps to moves") as u32
                })
                .collect()
        })
        .collect()
}

#[test]
fn size_three_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1, 2, 3],
        vec![8, 9, 4],
        vec![7, 6, 5]
    ];

    assert_eq!(spiral_matrix(3), expected);
}

#[test]
fn size_four_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1,  2,  3,  4],
        vec![12, 13, 14, 5],
        vec![11, 16, 15, 6],
        vec![10, 9,  8,  7],
    ];

    assert_eq!(spiral_matrix(4), expected);
}

#[test]
fn size_five_spiral() {
    #[rustfmt::skip]
    let expected: Vec<Vec<u32>> = vec![
        vec![1,  2,  3,  4,  5],
        vec![16, 17, 18, 19, 6],
        vec![15, 24, 25, 20, 7],
        vec![14, 23, 22, 21, 8],
        vec![13, 12, 11, 10, 9],
    ];

    assert_eq!(spiral_matrix(5), expected);
}
