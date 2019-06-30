#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_left: u16,
    rolls_left: usize,
    score: u16,
    spares: Vec<u8>,
}

const PINS_PER_FRAME: u16 = 10;

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            pins_left: PINS_PER_FRAME,
            rolls_left: 20,
            score: 0,
            spares: vec![],
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

        if self.spares.pop().is_some() {
            self.score += pins;
        }

        self.pins_left -= pins;
        self.rolls_left -= 1;

        self.score += pins;

        let is_frame_complete = self.rolls_left % 2 == 0;
        if is_frame_complete {
            if self.pins_left == 0 {
                self.spares.push(0);
            }
            
            self.pins_left = PINS_PER_FRAME;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.rolls_left {
            0 => Some(self.score),
            _ => None,
        }
    }
}
