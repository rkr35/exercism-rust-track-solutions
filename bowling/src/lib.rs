#![warn(clippy::pedantic)]
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const NUM_FRAMES: usize = 10;
const NUM_PINS_PER_FRAME: u16 = 10;

#[derive(Debug)]
enum FillBillKind {
    Spare,
    Strike,
}

#[derive(Debug)]
enum FrameKind {
    Normal,
    Spare,
    Strike,
    FillBill(FillBillKind)
}

impl Default for FrameKind { fn default() -> Self { FrameKind::Normal } }

#[derive(Default)]
struct Frame {
    first_roll: Option<u16>,
    second_roll: Option<u16>,
}

pub struct BowlingGame {
    frames: [Frame; NUM_FRAMES + 1],
    frame_cursor: usize,
    frames_left: usize,
    next_kind: FrameKind,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Default::default(),
            frame_cursor: 0,
            frames_left: NUM_FRAMES,
            next_kind: FrameKind::default(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames_left == 0 {
            return Err(Error::GameComplete);
        }

        let current_frame = &mut self.frames[self.frame_cursor];
        
        let is_fill_bill = if let FrameKind::FillBill(_) = self.next_kind { true } else { false };

        if let Some(first_roll) = current_frame.first_roll {
            let pins_left = NUM_PINS_PER_FRAME - first_roll % NUM_PINS_PER_FRAME;

            match pins.cmp(&pins_left) {
                Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                Ordering::Equal if !is_fill_bill => { self.next_kind = FrameKind::Spare; },
                _ => (),
            }
            
            current_frame.second_roll = Some(pins);

            self.complete_frame();
        } else {
            if pins > NUM_PINS_PER_FRAME {
                return Err(Error::NotEnoughPinsLeft);
            }

            *current_frame = Frame {
                first_roll: Some(pins),
                second_roll: None,
            };

            if let FrameKind::FillBill(FillBillKind::Spare) = self.next_kind {
                self.complete_frame()
            } else if !is_fill_bill && pins == NUM_PINS_PER_FRAME {
                self.next_kind = FrameKind::Strike;
                self.complete_frame();
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames_left > 0 {
            return None;
        }

        let mut sum = 0;
            
        for i in 0..NUM_FRAMES {
            let frame = &self.frames[i];
            let first = frame.first_roll.unwrap();
            let second = frame.second_roll.unwrap_or(0);

            let frame_score = first + second;
            sum += frame_score;

            if frame_score == NUM_PINS_PER_FRAME {
                let next_frame = &self.frames[i + 1];
                sum += next_frame.first_roll.unwrap();

                let is_strike = first == NUM_PINS_PER_FRAME;

                if is_strike {
                    if let Some(second) = next_frame.second_roll {
                        sum += second;
                    } else {
                        sum += self.frames[i + 2].first_roll.unwrap();
                    }
                }
            }
        }
        
        Some(sum)
    }

    fn complete_frame(&mut self) {
        self.frames_left -= 1;

        if self.frames_left == 0 {
            use FrameKind::*;

            match self.next_kind {
                Spare => {
                    self.next_kind = FillBill(FillBillKind::Spare);
                    self.frames_left = 1;
                }

                Strike => {
                    self.next_kind = FillBill(FillBillKind::Strike);
                    self.frames_left = 1;
                }

                _ => (),
            };
        } else {
            self.next_kind = FrameKind::default();
        }

        self.frame_cursor += 1;
    }
}
