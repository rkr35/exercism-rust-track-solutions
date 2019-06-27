#![warn(clippy::pedantic)]

pub fn abbreviate(phrase: &str) -> String {
    let mut previous = ' ';
    let mut abbreviations = vec![];

    for c in phrase.chars() {
        if "- ".contains(previous) || c.is_uppercase() && previous.is_lowercase() {
            abbreviations.push(c.to_ascii_uppercase());
        }

        previous = c;
    }

    abbreviations.into_iter().collect()
}
