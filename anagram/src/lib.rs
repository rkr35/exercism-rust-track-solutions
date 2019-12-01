#![warn(clippy::pedantic)]
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();

    let word_sorted = {
        let mut w: Vec<_> = word.clone().chars().collect();
        w.sort_unstable();
        w
    };

    possible_anagrams
        .iter()
        .copied()
        .filter(|possible| {
            let possible = possible.to_lowercase();
            possible.len() == word.len() && word != possible && {
                let mut possible: Vec<_> = possible.chars().collect();
                possible.sort_unstable();
                possible == word_sorted                
            }
        })
        .collect()
}
