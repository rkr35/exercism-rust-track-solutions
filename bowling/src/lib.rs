#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Frame {
    Open(OpenKind),
    Spare(SpareKind),
    Strike(StrikeKind),
}

enum OpenKind {
    OneRoll(u16),
    TwoRolls(u16, u16),
}

enum SpareKind {
    Empty,
    NextRoll(u16),
}

enum StrikeKind {
    Empty,
    NextRoll(u16),
    NextTwoRolls(u16, u16),
}

const PINS_PER_FRAME: u16 = 10;

pub struct BowlingGame {
    frames
}


impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
       
    }

    pub fn score(&self) -> Option<u16> {

    }
}
