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
    Empty,
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
    last_frame: Frame,
    frames_left: usize,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            last_frame: Frame::Open(OpenKind::Empty),
            frames_left: 10,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        use Frame::*;

        match self.last_frame {
            Open(OpenKind::Empty) => {
                if pins > PINS_PER_FRAME {
                    Err(Error::NotEnoughPinsLeft)
                } else {
                    self.last_frame = if pins == PINS_PER_FRAME {
                        self.frames_left -= 1;
                        Strike(StrikeKind::Empty)
                    } else {
                        Open(OpenKind::OneRoll(pins))
                    };

                    Ok(())
                }
            },
            _ => unimplemented!("This case hasn't been implemented yet."),
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames_left {
            0 => Some(self.score),
            _ => None,
        }
    }
}
