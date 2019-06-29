#![warn(clippy::pedantic)]

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<isize> for Direction {
    fn from(discriminant: isize) -> Self {
        use Direction::*;
        const fn ordinal(d: Direction) -> isize { d as isize }
        const NUM_DIRECTIONS: isize = 1 + ordinal(West);
        
        let my_mod = |a, b| (a % b + b) % b;

        match my_mod(discriminant, NUM_DIRECTIONS) {
            o if o == ordinal(North) => North,
            o if o == ordinal(East) => East,
            o if o == ordinal(South) => South,
            o if o == ordinal(West) => West,
            _ => panic!("unrecognized direction discriminant: {}", discriminant),
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
