#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::collections::{HashMap, HashSet};
use std::iter::once;

macro_rules! get_digits {
    ($number:expr, $num_digits:expr) => {
        (0..$num_digits)
            .map(move |digit_index| $number / 10_usize.pow(digit_index as u32) % 10)
            .rev()
    };
}

fn test_permutation(letters: &[(char, isize)], permutation: usize, first_letters: &HashSet<char>) -> Option<HashMap<char, u8>> {
    let mut seen_digits = [false; 10];
    let mut letter_digit_pairs: [(char, u8); 10] = [Default::default(); 10];
    let mut cursor = 0;

    let mut solution_value = 0;

    for (i, digit) in get_digits!(permutation, letters.len()).enumerate() {
        let (letter, constant) = letters[i];

        if seen_digits[digit] || digit == 0 && first_letters.contains(&letter) {
            return None;
        }

        seen_digits[digit] = true;
        solution_value += constant * digit as isize;
        letter_digit_pairs[cursor] = (letter, digit as u8);
        cursor += 1;
    }

    if solution_value == 0 {
        Some(letter_digit_pairs[0..cursor].iter().copied().collect())
    } else {
        None
    }
}

fn find_solution(letters: &[(char, isize)], first_letters: &HashSet<char>) -> Option<HashMap<char, u8>> {
    (
        (0..letters.len()).fold(0, |acc, current| acc * 10 + current) 
        ..=
        (0..letters.len()).fold(0, |acc, current| acc * 10 + 9 - current)
    )
    
    .find_map(|permutation| test_permutation(&letters, permutation, first_letters))
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut operands: Vec<_> = input.split(" + ").collect();
    let last_two: Vec<_> = operands.last()?.split(" == ").collect();
    *operands.last_mut()? = last_two.get(0)?;
    let result = last_two.get(1)?;

    let constants: Vec<_> = {
        let mut constants = HashMap::new();

        let mut add_to_constants = |word: &str, direction: isize| {
            for (i, letter) in word.chars().rev().enumerate() {
                *constants.entry(letter).or_default() += direction * 10_isize.pow(i as u32);
            }
        };

        for operand in &operands {
            add_to_constants(operand, 1)
        }

        add_to_constants(result, -1);

        constants.into_iter().collect()
    };

    let first_letters = operands
        .iter()
        .chain(once(result))
        .map(|word| {
            word.chars()
                .nth(0)
                .expect("could not obtain first character of a word")
        })
        .collect();

    find_solution(&constants, &first_letters)
}
