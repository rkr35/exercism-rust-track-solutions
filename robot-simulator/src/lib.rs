#![warn(clippy::pedantic)]

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    fn turn(self, units: isize) -> Self {
        Self {
            direction: {
                use Direction::*;
                let m = |a, b| (a % b + b) % b;
                [North, East, South, West][m(self.direction as isize + units, 1 + West as isize) as usize]
            },
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
            position: {
                let movement = [(0, 1), (1, 0), (0, -1), (-1, 0)][self.direction as usize];
                (self.position.0 + movement.0, self.position.1 + movement.1)
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instruction| match instruction {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => panic!("unrecognized instruction: \"{}\"", instruction),
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
