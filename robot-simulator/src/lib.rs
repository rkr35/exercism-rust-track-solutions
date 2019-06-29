#![warn(clippy::pedantic)]
use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<isize> for Direction {
    fn from(discriminant: isize) -> Self {
        use Direction::*;
        let my_mod = |a, b| (a % b + b) % b;
        [North, East, South, West][my_mod(discriminant, 1 + West as isize) as usize]
    }
}

struct Position<T>(T, T);

impl<T> Add<(T, T)> for Position<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Robot {
    position: Position<i32>,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: Position(x, y),
            direction: d,
        }
    }

    fn turn(self, units: isize) -> Self {
        Self {
            direction: Direction::from(self.direction as isize + units),
            ..self
        }
    }

    pub fn turn_right(self) -> Self {
        self.turn(1)
    }

    pub fn turn_left(self) -> Self {
        self.turn(-1)
    }

    pub fn advance(self) -> Self {
        Self {
            position: self.position + [(0, 1), (1, 0), (0, -1), (-1, 0)][self.direction as usize],
            ..self
        }
    }

    fn do_instruction(self, instruction: char) -> Self {
        match instruction {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            'A' => self.advance(),
            _ => panic!("unrecognized instruction: \"{}\"", instruction),
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, Self::do_instruction)
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.0, self.position.1)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
