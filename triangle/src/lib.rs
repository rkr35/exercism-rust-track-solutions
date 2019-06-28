#![warn(clippy::pedantic)]

use std::iter::{empty, Sum};
use std::ops::Add;

pub struct Triangle<T>([T; 3]);

impl<T> Triangle<T>
where
    T: Copy + Add + Sum<T> + PartialOrd<T> + PartialOrd<<T as Add>::Output> 
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        let sum: T = sides.iter().cloned().sum();
        let &max = sides.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        if sum == empty().sum::<T>() || sum < (max + max) {
            return None;
        }

        Some(Self(sides))
    }

    fn number_sides_equal(&self) -> usize {
        let [side1, side2, side3] = self.0;

        let mut num = 0;

        if side1 == side2 {
            num += 1;
        }

        if side1 == side3 {
            num += 1;
        }

        if side2 == side3 {
            num += 1;
        }

        num
    }

    pub fn is_equilateral(&self) -> bool {
        self.number_sides_equal() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.number_sides_equal() >= 1
    }

    pub fn is_scalene(&self) -> bool {
        self.number_sides_equal() == 0
    }
}
