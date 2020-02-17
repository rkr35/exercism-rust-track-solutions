use std::cmp::{Ord, Ordering};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Ordering::{Less, Greater, Equal as Eq};
    use Comparison::*;

    let len_cmp = first_list
        .len()
        .cmp(&second_list.len());

    let [smaller, larger] = if let Greater = len_cmp {
        [second_list, first_list]
    } else {
        [first_list, second_list]
    };

    let is_smaller_sublist_of_larger = (0..=larger.len() - smaller.len())
        .any(|i| larger[i..]
            .iter()
            .zip(smaller.iter())
            .all(|(l, s)| l == s));
        
    if is_smaller_sublist_of_larger {
        match len_cmp {
            Less => Sublist,
            Greater => Superlist,
            Eq => Equal,
        }
    } else {
        Unequal
    }
}
