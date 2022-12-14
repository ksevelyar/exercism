const STRIKE: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<[Option<u16>; 3]>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        };

        if pins > STRIKE {
            return Err(Error::NotEnoughPinsLeft);
        };

        if self.frames.is_empty() {
            self.frames.push([Some(pins), None, None]);
            return Ok(());
        };

        let last_frame = self.frames.last().unwrap();

        let is_last_frame_closed = match last_frame {
            [Some(10), _, _] => self.frames.len() != 10,
            [Some(a), Some(b), _] => !(self.frames.len() == 10 && a + b == 10),
            _ => false,
        };

        if is_last_frame_closed {
            self.frames.push([Some(pins), None, None]);
            return Ok(());
        };

        match self.frames.as_slice() {
            [head @ .., last] => {
                let tail = match *last {
                    [Some(pins0), None, None] if pins0 + pins > 10 && pins0 != 10 => {
                        return Err(Error::NotEnoughPinsLeft)
                    }
                    [Some(pins0), None, None] => [Some(pins0), Some(pins), None],
                    [Some(10), Some(pins1), None]
                        if (pins + pins1 > 10 || pins == 10) && pins1 != 10 =>
                    {
                        return Err(Error::NotEnoughPinsLeft)
                    }
                    [Some(pins0), Some(pins1), None] => [Some(pins0), Some(pins1), Some(pins)],
                    _ => [Some(pins), None, None],
                };
                self.frames = head.to_vec();
                self.frames.push(tail);

                return Ok(());
            }
            _ => (),
        }

        Ok(())
    }

    fn score_frame(frame: Option<&[Option<u16>; 3]>) -> u16 {
        match frame {
            Some(frame) => frame[0].unwrap_or(0) + frame[1].unwrap_or(0) + frame[2].unwrap_or(0),
            None => 0,
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        };

        self.frames
            .iter()
            .enumerate()
            .map(|(ind, frame)| {
                let frame_pins = frame.iter().flatten().sum();
                let frame_throws = frame.iter().flatten().count();

                match (frame_throws, frame_pins) {
                    (1, 10) => {
                        let next_frame = self.frames.get(ind + 1).unwrap_or(&[None, None, None]);

                        let sum = match next_frame[1] {
                            None => {
                                next_frame[0].unwrap()
                                    + self.frames.get(ind + 2).unwrap_or(&[None, None, None])[0]
                                        .unwrap_or(0)
                            }
                            _ => next_frame[0].unwrap() + next_frame[1].unwrap(),
                        };

                        Some(10 + sum)
                    }
                    (_, 10) => {
                        let next_frame = self.frames.get(ind + 1).unwrap_or(&[None, None, None]);

                        Some(10 + next_frame[0].unwrap())
                    }
                    _ => Some(frame_pins),
                }
            })
            .into_iter()
            .sum()
    }

    fn is_complete(&self) -> bool {
        self.frames.len() == 10
            && match self.frames[9] {
                [Some(_), None, None] => false,
                [Some(10), _, None] => false,
                [Some(a), Some(b), None] => a + b != 10,
                _ => true,
            }
    }
}

#[test]
fn ten_frames_without_a_strike_or_spare() {
    let mut game = BowlingGame::new();

    for _ in 0..10 {
        let _ = game.roll(3);

        let _ = game.roll(6);
    }

    assert_eq!(game.score(), Some(90));
}

#[test]
fn points_scored_in_the_two_rolls_after_a_strike_are_counted_twice_as_a_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    let _ = game.roll(5);

    let _ = game.roll(3);

    for _ in 0..16 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(26));
}

#[test]
fn a_strike_earns_ten_points_in_a_frame_with_a_single_roll() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(10));
}

#[test]
fn all_strikes_is_a_perfect_score_of_300() {
    let mut game = BowlingGame::new();

    for _ in 0..12 {
        let _ = game.roll(10);
    }

    assert_eq!(game.score(), Some(300));
}

#[test]
fn if_the_last_frame_is_a_strike_you_cannot_score_before_the_extra_rolls_are_taken() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert_eq!(game.score(), None);

    let _ = game.roll(10);

    assert!(game.score().is_some());
}

#[test]
fn a_spare_with_the_two_roll_bonus_does_not_get_a_bonus_roll() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    let _ = game.roll(7);

    let _ = game.roll(3);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn consecutive_strikes_each_get_the_two_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(10);

    let _ = game.roll(10);

    let _ = game.roll(10);

    let _ = game.roll(5);

    let _ = game.roll(3);

    for _ in 0..12 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(81));
}

#[test]
fn a_strike_with_the_one_roll_bonus_after_a_spare_in_the_last_frame_does_not_get_a_bonus() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(7);

    let _ = game.roll(3);

    let _ = game.roll(10);

    assert_eq!(game.score(), Some(20));
}

#[test]
fn consecutive_spares_each_get_a_one_roll_bonus() {
    let mut game = BowlingGame::new();

    let _ = game.roll(5);

    let _ = game.roll(5);

    let _ = game.roll(3);

    let _ = game.roll(7);

    let _ = game.roll(4);

    for _ in 0..15 {
        let _ = game.roll(0);
    }

    assert_eq!(game.score(), Some(31));
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_be_a_non_strike_followed_by_a_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(6).is_ok());

    assert_eq!(game.roll(10), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_cannot_score_an_invalid_number_of_pins() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(5).is_ok());

    assert_eq!(game.roll(6), Err(Error::NotEnoughPinsLeft));
}

#[test]
fn the_two_balls_after_a_final_strike_can_be_a_strike_and_non_strike() {
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    let _ = game.roll(10);

    assert!(game.roll(10).is_ok());

    assert!(game.roll(6).is_ok());
}
