#![warn(clippy::pedantic)]

use std::iter::{empty, Sum};
use std::ops::Add;

const NUM_SIDES: usize = 3;

pub struct Triangle<T>([T; NUM_SIDES]);

impl<T> Triangle<T>
where
    T: Copy + Add + Sum<T> + PartialOrd<T> + PartialOrd<<T as Add>::Output> 
{
    pub fn build(sides: [T; NUM_SIDES]) -> Option<Self> {
        let sum: T = sides.iter().cloned().sum();
        let &max = sides.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        if sum == empty().sum::<T>() || sum < (max + max) {
            return None;
        }

        Some(Self(sides))
    }

    fn number_sides_equal(&self) -> usize {
        let last_index = self.0.len() - 1;

        (0..last_index)
            .map(|i| (i+1..=last_index).map(|j| usize::from(self.0[i] == self.0[j])).sum::<usize>())
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
