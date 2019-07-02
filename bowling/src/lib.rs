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
enum FrameKind {
    Normal,
    Spare,
    Strike,
    FillBillSpare,
    FillBillStrike,
}

impl Default for FrameKind { fn default() -> Self { FrameKind::Normal } }

#[derive(Default)]
struct Frame {
    first_roll: Option<u16>,
    second_roll: Option<u16>,
    kind: FrameKind
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
        use FrameKind::*;
        dbg!(self.frames_left);
        dbg!(pins);

        if self.frames_left == 0 {
            return Err(Error::GameComplete);
        }

        let current_frame = &mut self.frames[self.frame_cursor];

        if let Some(first_roll) = current_frame.first_roll {
            let pins_left = NUM_PINS_PER_FRAME - first_roll;

            dbg!(pins_left);

            dbg!(&self.next_kind);

            match current_frame.kind {
                FillBillStrike => (),
                _ => match pins.cmp(&pins_left) {
                    Ordering::Greater => return Err(Error::NotEnoughPinsLeft),
                    Ordering::Equal => self.next_kind = Spare,
                    _ => (),
                }
            };
                
            current_frame.second_roll = Some(pins);

            self.complete_frame();
        } else {
            if pins > NUM_PINS_PER_FRAME {
                return Err(Error::NotEnoughPinsLeft);
            }

            *current_frame = Frame {
                first_roll: Some(pins),
                second_roll: None,
                kind: std::mem::replace(&mut self.next_kind, FrameKind::default())
            };

            dbg!(&current_frame.kind);

            if let FillBillSpare = current_frame.kind {
                self.complete_frame();
            } else if pins == NUM_PINS_PER_FRAME {
                if let FillBillStrike = current_frame.kind {} 
                else {
                    dbg!("strike");
                    self.next_kind = Strike;
                    self.complete_frame();
                };
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames_left > 0 {
            return None;
        }

        Some(
            self.frames.iter()
                .take(self.frame_cursor)
                .map(|frame| {
                    use FrameKind::*;

                    let first = frame.first_roll.unwrap();
                    let second = frame.second_roll.unwrap_or(0);

                    match frame.kind {
                        Spare => 2*first + second,
                        Strike => 2*(first + second),
                        FillBillSpare => first,
                        FillBillStrike | Normal => first + second,
                    }
                })
                .sum(),
        )
    }

    fn complete_frame(&mut self) {
        self.frames_left -= 1;

        if self.frames_left == 0 {
            use FrameKind::*;

            dbg!(&self.next_kind);

            match self.next_kind {
                Spare => {
                    self.next_kind = FillBillSpare;
                    self.frames_left += 1;
                }

                Strike => {
                    self.next_kind = FillBillStrike;
                    self.frames_left += 1;
                }

                _ => ()
            };
        }

        self.frame_cursor += 1;
    }
}
