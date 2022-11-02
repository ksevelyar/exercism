#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        let new_direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Robot {
            direction: new_direction,
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        let new_direction = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };

        Robot {
            direction: new_direction,
            ..self
        }
    }

    pub fn advance(self) -> Self {
        use Direction::*;
        let new_coords = match self.direction {
            North => (self.x, self.y + 1),
            West => (self.x - 1, self.y),
            South => (self.x, self.y - 1),
            East => (self.x + 1, self.y),
        };

        Robot {
            x: new_coords.0,
            y: new_coords.1,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Robot {
        instructions.chars().fold(self, |acc, char| match char {
            'R' => acc.turn_right(),
            'L' => acc.turn_left(),
            'A' => acc.advance(),
            _ => panic!("Action is unknown: {}", char),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

#[test]
fn turning_right_from_south_points_the_robot_west() {
    let robot = Robot::new(0, 0, Direction::South).turn_right();

    assert_eq!(&Direction::West, robot.direction());
}

#[test]
fn follow_instructions_to_move_east_and_north() {
    let robot = Robot::new(8, 4, Direction::South).instructions("LAAARRRALLLL");

    assert_eq!((11, 5), robot.position());

    assert_eq!(&Direction::North, robot.direction());
}
