#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const NUM_FRAMES: usize = 10;
const NUM_PINS_PER_FRAME: u16 = 10;

#[derive(Default)]
struct Frame {
    first_roll: Option<u16>,
    second_roll: Option<u16>,
}

pub struct BowlingGame {
    frames: [Frame; NUM_FRAMES],
    frame_cursor: usize,
    frames_left: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Default::default(),
            frame_cursor: 0,
            frames_left: NUM_FRAMES,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames_left == 0 {
            return Err(Error::GameComplete);
        }

        let current_frame = &mut self.frames[self.frame_cursor];

        if let Some(first_roll) = current_frame.first_roll {
            let pins_left = NUM_PINS_PER_FRAME - first_roll;

            if pins > pins_left {
                return Err(Error::NotEnoughPinsLeft);
            }

            current_frame.second_roll = Some(pins);

            self.frames_left -= 1;
            self.frame_cursor += 1;
        } else {
            if pins > NUM_PINS_PER_FRAME {
                return Err(Error::NotEnoughPinsLeft);
            }

            *current_frame = Frame {
                first_roll: Some(pins),
                second_roll: None,
            };
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames_left > 0 {
            return None;
        }

        Some(
            self.frames
                .iter()
                .map(|Frame {first_roll, second_roll}| first_roll.unwrap() + second_roll.unwrap())
                .sum(),
        )
    }
}
