#![warn(clippy::all)]
#![warn(clippy::pedantic)]

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    const NUM_ENGLISH_DIGITS: usize = 10;
    const NUM_ENGLISH_LETTERS: usize = 26;
    const POINTS: [u8; NUM_ENGLISH_LETTERS] = [
        1,3,3,2,  // abcd
        1,4,2,4,  // efgh
        1,8,5,1,  // ijkl
        3,1,1,3,  // mnop
        10,1,1,1, // qrst
        1,4,4,8,  // uvwx
        4,10      // yz
    ];

    word
        .chars()
        .filter_map(|character| character
            .to_digit((POINTS.len() + NUM_ENGLISH_DIGITS) as u32)
            .map(|digit| u64::from(POINTS[digit as usize - NUM_ENGLISH_DIGITS])))
        .sum()
}
