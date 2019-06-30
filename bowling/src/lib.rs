#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_left: u16,
    rolls_left: usize,
    score: u16,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            pins_left: 10,
            rolls_left: 20,
            score: 0,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.rolls_left == 0 {
            return Err(Error::GameComplete);
        }

        self.rolls_left -= 1;

        self.score += pins;

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.rolls_left {
            0 => Some(self.score),
            _ => None,
        }
    }
}
