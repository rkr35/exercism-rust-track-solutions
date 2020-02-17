#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    let sub = |l: &[T], s: &[T]| l
        .windows(if s.len() == 0 { return true; } else { s.len() })
        .any(|w| w == s);
    match [first_list.len(), second_list.len()] {
        [f, s] if f == s => if first_list == second_list { Equal } else { Unequal },
        [f, s] if f < s => if sub(second_list, first_list) { Sublist } else { Unequal },
        [f, s] if f > s => if sub(first_list, second_list) { Superlist } else { Unequal },
        [f, s] => unreachable!("|first| = {}, |second| = {}", f, s),
    }
}
