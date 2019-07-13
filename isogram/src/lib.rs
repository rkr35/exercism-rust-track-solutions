#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // todo: Is there a way to get rid of this heap allocation?
    let mut seen_letters = HashSet::with_capacity(candidate.len());

    candidate
        .chars()
        .filter(|&c| c.is_alphabetic())
        .all(|letter| seen_letters.insert(letter.to_ascii_lowercase()))
}
