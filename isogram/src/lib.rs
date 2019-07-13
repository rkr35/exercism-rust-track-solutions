#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub fn check(candidate: &str) -> bool {
    const ALPHABET_SIZE: usize = 26;
    const NUM_UNIQUE_DIGITS: usize = 10;
    const ENGLISH_RADIX: u32 = (ALPHABET_SIZE + NUM_UNIQUE_DIGITS) as u32;

    let mut seen_letters = [false; ALPHABET_SIZE];

    candidate
        .chars()
        .filter_map(|character| character.to_digit(ENGLISH_RADIX))
        .all(|digit| {
            let seen_letters_index = digit as usize - NUM_UNIQUE_DIGITS;
            let already_seen = seen_letters[seen_letters_index];

            if already_seen {
                false
            } else {
                seen_letters[seen_letters_index] = true;
                true
            }
        })
}
