#![warn(clippy::pedantic)]

const NUM_FRAMES: usize = 10;
const NUM_PINS_PER_FRAME: u16 = 10;


#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

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
    FillBill(FillBillKind),
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
        if self.frames_left == 0 { return Err(Error::GameComplete); }
        if let Some(first_roll) = self.get_current_frame().first_roll { self.handle_second_roll(first_roll, pins)?; } 
        else { self.handle_first_roll(pins)?; }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames_left == 0 { Some((0..NUM_FRAMES).map(|i| self.get_frame_score(i)).sum()) }
        else { None }
    }

    fn get_first_roll_after_next_frame(&self, frame_index: usize) -> u16 {
        self.frames[frame_index + 2].first_roll.unwrap()
    }

    fn get_spare_or_strike_bonus_score(&self, frame_index: usize, is_strike: bool) -> u16 {
        let next_frame = &self.frames[frame_index + 1];
        let score = next_frame.first_roll.unwrap();

        if !is_strike { return score; }

        score + if let Some(second) = next_frame.second_roll { second } 
                else { self.get_first_roll_after_next_frame(frame_index) }
    }

    fn get_frame_score(&self, frame_index: usize) -> u16 {
        let Frame { first_roll, second_roll, .. } = &self.frames[frame_index];
        let first = first_roll.unwrap();
        let score = first + second_roll.unwrap_or(0);
        let is_spare_or_strike = score == NUM_PINS_PER_FRAME;

        score + if is_spare_or_strike {
            let is_strike = first == NUM_PINS_PER_FRAME;
            self.get_spare_or_strike_bonus_score(frame_index, is_strike)
        } else {
            0
        }
    }

    fn handle_second_roll(&mut self, first_roll: u16, pins: u16) -> Result<(), Error> {
        let pins_left = NUM_PINS_PER_FRAME - first_roll % NUM_PINS_PER_FRAME;
        if pins > pins_left { return Err(Error::NotEnoughPinsLeft); } 
        else if pins == pins_left && !self.is_fill_bill() { self.next_kind = FrameKind::Spare; }
        self.get_current_frame().second_roll = Some(pins);
        self.complete_frame();
        Ok(())
    }

    fn handle_first_roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > NUM_PINS_PER_FRAME { return Err(Error::NotEnoughPinsLeft); }
        self.initialize_current_frame(pins);
        self.complete_frame_if_necessary_after_first_roll(pins);
        Ok(())
    }

    fn is_fill_bill(&self) -> bool {
        if let FrameKind::FillBill(_) = self.next_kind { true } 
        else { false }
    }

    fn get_current_frame(&mut self) -> &mut Frame {
        &mut self.frames[self.frame_cursor]
    }

    fn initialize_current_frame(&mut self, first_roll_pins: u16) {
        *self.get_current_frame() = Frame {
            first_roll: Some(first_roll_pins),
            second_roll: None,
        };
    }

    fn complete_frame_if_necessary_after_first_roll(&mut self, first_roll_pins: u16) {
        if let FrameKind::FillBill(FillBillKind::Spare) = self.next_kind {
            self.complete_frame()
        } else if first_roll_pins == NUM_PINS_PER_FRAME && !self.is_fill_bill() {
            self.next_kind = FrameKind::Strike;
            self.complete_frame();
        }
    }

    fn set_to_fill_bill_if_need_be(&mut self) {
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
    }

    fn adjust_next_kind(&mut self) {
        if self.frames_left == 0 { self.set_to_fill_bill_if_need_be(); } 
        else { self.next_kind = FrameKind::default(); }
    }

    fn complete_frame(&mut self) {
        self.frames_left -= 1;
        self.frame_cursor += 1;
        self.adjust_next_kind();
    }
}
