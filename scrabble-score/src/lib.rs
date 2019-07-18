#![warn(clippy::all)]
#![warn(clippy::pedantic)]

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word
        .chars()
        .map(|character| match character.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' 
                => 1,

            'd' | 'g' 
                => 2,

            'b' | 'c' | 'm' | 'p'
                => 3,

            'f' | 'h' | 'v' | 'w' | 'y' 
                => 4,

            'k' 
                => 5,

            'j' | 'x'
                => 8,

            'q' | 'z'
                => 10,
            
            _ => 0,
        })
        .sum()
}
