#![warn(clippy::pedantic)]
use std::mem::replace;

pub fn abbreviate(phrase: &str) -> String {
    let mut previous = ' ';

    phrase
        .chars()
        .filter_map(|c| {
            let previous = replace(&mut previous, c);

            if "- ".contains(previous) || c.is_uppercase() && previous.is_lowercase() {
                Some(c.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}
