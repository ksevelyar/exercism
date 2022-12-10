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

        let last_frame = self
            .frames
            .last().unwrap();

        let is_last_frame_closed = match last_frame {
            [Some(10), _, _] => true,
            [Some(_), Some(_), _] => true,
            _ => false
        };
        if is_last_frame_closed {
            self.frames.push([Some(pins), None, None]);
            return Ok(());
        };

        match self.frames.as_slice() {
            [head @ .., last] => {
                let tail = match *last {
                    [Some(pins0), None, None] => [Some(pins0), Some(pins), None],
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
                let frame_pins = Self::score_frame(Some(frame));

                match (ind, frame_pins) {
                    (_, 10) if ind < 9 => Some(
                        STRIKE
                            + Self::score_frame(self.frames.get(ind + 1))
                            + Self::score_frame(self.frames.get(ind + 2)),
                    ),
                    _ => Some(frame_pins),
                }
            })
            .sum()
    }

    fn is_complete(&self) -> bool {
        self.frames.len() == 10
    }
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

    dbg!(&game);
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
