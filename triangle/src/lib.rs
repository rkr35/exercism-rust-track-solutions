#![warn(clippy::pedantic)]

use std::iter::{empty, Sum};
use std::ops::Add;
use std::cmp::Ordering;

const NUM_SIDES: usize = 3;

pub struct Triangle<T>([T; NUM_SIDES]);

impl<T> Triangle<T>
where
    T: Copy + Add<Output=T> + Sum + PartialOrd
{
    pub fn build(sides: [T; NUM_SIDES]) -> Option<Self> {
        let sum: T = sides.iter().cloned().sum();
        let &max = sides.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;

        if sum == empty().sum::<T>() || sum < (max + max) {
            None
        } else {
            Some(Self(sides))
        }
    }

    fn number_sides_equal(&self) -> usize {
        let last_index = self.0.len() - 1;

        (0..last_index)
            .flat_map(|i| (i+1..=last_index).map(move |j| usize::from(self.0[i] == self.0[j])))
            .sum()
    }

    pub fn is_equilateral(&self) -> bool {
        self.number_sides_equal() == NUM_SIDES
    }

    pub fn is_isosceles(&self) -> bool {
        self.number_sides_equal() >= 1
    }

    pub fn is_scalene(&self) -> bool {
        self.number_sides_equal() == 0
    }
}
