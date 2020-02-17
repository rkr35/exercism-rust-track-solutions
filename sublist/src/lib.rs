#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;
    let sublist = |larger: &[T], smaller: &[T]| larger
        .windows(if smaller.len() == 0 { return true; } else { smaller.len() })
        .any(|w| w == smaller);

    match [first_list.len(), second_list.len()] {
        [f, s] if f == s => if first_list == second_list { Equal } else { Unequal },
        [f, s] if f < s => if sublist(second_list, first_list) { Sublist } else { Unequal },
        [f, s] if f > s => if sublist(first_list, second_list) { Superlist } else { Unequal },
        [f, s] => unreachable!("|first| = {}, |second| = {}", f, s),
    }
}
