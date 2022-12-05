const STRIKE: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    throws: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { throws: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        };

        if pins > STRIKE {
            return Err(Error::NotEnoughPinsLeft);
        };

        self.throws.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        };

        self.throws
            .iter()
            .enumerate()
            .map(|(ind, pins)| match pins {
                _ if *pins == STRIKE => {
                    Some(STRIKE + self.throws.get(ind + 1)? + self.throws.get(ind + 2)?)
                }
                _ => Some(*pins),
            })
            .sum()
    }

    fn is_complete(&self) -> bool {
        self.throws.len() == 19
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
