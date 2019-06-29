#![warn(clippy::pedantic)]

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
            _ => panic!("unexpected direction discriminant: {}", discriminant),
        }
    }
}

struct Position<T> {
    x: T,
    y: T,
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
        unimplemented!()
    }

    pub fn instructions(self, instructions: &str) -> Self {
        unimplemented!(
            "Follow the given sequence of instructions: {}",
            instructions
        )
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
