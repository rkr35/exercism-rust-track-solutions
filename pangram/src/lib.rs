#![no_std]

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    const SEEN_ALL_LETTERS: u32 = (1 << 26) - 1;
    let mut seen_letters: u32 = 0;

    sentence
        .chars()
        .filter_map(|character| character
            .to_digit(36)                 // 10 digits + 26 letters yields radix of 36
            .filter(|&digit| digit >= 10) // ignore digits
            .map(|digit| digit - 10))     // a=0, b=1, c=2, ..., z=25
        .for_each(|digit| { seen_letters |= 1 << digit });

    seen_letters == SEEN_ALL_LETTERS
}
