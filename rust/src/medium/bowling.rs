#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    pins: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { pins: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        };

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        };

        self.pins.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        };

        Some(
            self.pins
                .iter()
                .enumerate()
                .map(|(ind, x)| match ind > 2 && self.pins[ind - 2] == 10 {
                    true => *x * 2,
                    false => *x,
                })
                .sum(),
        )
    }

    fn is_complete(&self) -> bool {
        self.pins.len() == 20
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
