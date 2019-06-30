#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_left: u16,
    rolls_left: usize,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            pins_left: 10,
            rolls_left: 20
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

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.rolls_left {
            0 => Some(0),
            _ => None,
        }
    }
}
