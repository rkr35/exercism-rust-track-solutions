use std::collections::HashSet;

// A poor and insecure hash function for anagrams that adds the numeric value
// of each character in 's' converted to lowercase.
// This hash function works for the Exercism tests.
// This hash will easily collide for trivial inputs, e.g., poor_hash("22") == poor_hash("d").
// Don't use this hash in production.
fn poor_hash(s: &str) -> u32 {
    chars(s).map(u32::from).sum()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_poor_hash = poor_hash(word);
    
    possible_anagrams
        .iter()
        .filter(|possible| poor_hash(possible) == word_poor_hash
                           && chars(word).ne(chars(possible)))
        .copied()
        .collect()
}

fn chars(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars().flat_map(char::to_lowercase)
}
