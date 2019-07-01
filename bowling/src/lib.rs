#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        None
    }
}
