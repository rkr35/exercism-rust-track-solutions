use std::collections::BTreeMap;
use std::iter::repeat;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h
        .iter()
        .flat_map(|(&points, characters)| characters.iter().map(char::to_ascii_lowercase).zip(repeat(points)))
        .collect()
}
