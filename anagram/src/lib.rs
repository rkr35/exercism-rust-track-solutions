#![warn(clippy::pedantic)]
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();

    let sort = |s: String| {
        let mut s: Vec<_> = s.chars().collect();
        s.sort_unstable();
        s
    };

    let word_sorted = sort(word.clone());

    possible_anagrams
        .iter()
        .copied()
        .filter(|possible| {
            let possible = possible.to_lowercase();
            possible.len() == word.len() 
                && word != possible 
                && sort(possible) == word_sorted
        })
        .collect()
}
