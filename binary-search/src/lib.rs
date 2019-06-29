use std::cmp::Ordering;
use std::mem::replace;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let (mut low, mut high) = (0, array.len());
    let mut middle = (low + high) / 2;

    loop {
        match key.cmp(&array[middle]) {
            Ordering::Equal => return Some(middle),
            Ordering::Less => high = middle,
            Ordering::Greater => low = middle,
        };

        if replace(&mut middle, (low + high) / 2) == middle {
            return None;
        }
    }
}
