#![warn(clippy::pedantic)]
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

const HOURS_PER_DAY: i64 = 24;
const MINUTES_PER_HOUR: i64 = 60;
const MINUTES_PER_DAY: i64 = MINUTES_PER_HOUR * HOURS_PER_DAY;

fn my_mod(a: i64, b: i64) -> i64 {
    (a % b + b) % b
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Self {
            minutes: my_mod(hours * MINUTES_PER_HOUR + minutes, MINUTES_PER_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR
        )
    }
}

// String::from(Clock)
impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        clock.to_string()
    }
}
