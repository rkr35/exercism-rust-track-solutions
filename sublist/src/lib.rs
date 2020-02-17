#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use std::cmp::Ordering::{Less, Equal as Equ, Greater};
    use Comparison::*;

    let sub = |l: &[T], s: &[T]| l
        .windows(if s.is_empty() { return true; } else { s.len() })
        .any(|w| w == s);

    match first_list.len().cmp(&second_list.len()) {
        Less => if sub(second_list, first_list) { Sublist } else { Unequal },
        Equ => if first_list == second_list { Equal } else { Unequal }
        Greater => if sub(first_list, second_list) { Superlist } else { Unequal },
    }
}
