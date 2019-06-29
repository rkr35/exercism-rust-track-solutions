#![warn(clippy::pedantic)]
use std::ops::Add;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North, // 0 -> +1
    East, // 1 -> +1
    South, // 2 -> -1
    West, // 3 -> -1
}

impl Direction {
    const fn ordinal(self) -> isize { self as isize }
    const fn last() -> Self { Direction::West }
    const fn num_directions() -> isize { 1 + Self::last().ordinal() }
}

impl From<isize> for Direction {
    fn from(discriminant: isize) -> Self {
        use Direction::*;
        let my_mod = |a, b| (a % b + b) % b;
        
        match my_mod(discriminant, Self::num_directions()) {
            o if o == North.ordinal() => North,
            o if o == East.ordinal() => East,
            o if o == South.ordinal() => South,
            o if o == West.ordinal() => West,
            _ => panic!("unexpected direction discriminant: \"{}\"", discriminant),
        }
    }
}

struct Position<T> {
    x: T,
    y: T,
}

impl<T, U, V> Add<(U, V)> for Position<T> 
    where T: Add<U, Output=T> + Add<V, Output=T> {
    type Output = Self;

    fn add(self, rhs: (U, V)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

pub struct Robot {
    position: Position<i32>,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: Position { x, y },
            direction: d,
        }
    }

    fn turn(self, units: isize) -> Self {
        Self {
            direction: Direction::from(self.direction.ordinal() + units),
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
        use Direction::*;

        let movement = match self.direction {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        Self {
            position: self.position + movement,
            ..self
        }
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for instruction in instructions.chars() {
            self = match instruction {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'A' => self.advance(),
                _ => panic!("unrecognized instruction: \"{}\"", instruction)
            }
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
