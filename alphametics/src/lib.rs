#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::collections::{HashMap};
use std::convert::TryFrom;

macro_rules! get_digits {
    ($number:expr, $num_digits:expr) => {
        (0..$num_digits).map(|digit_index| $number / 10_usize.pow(digit_index as u32) % 10)
    };
}

fn has_unique_digits(number_to_check: usize,
                     number_of_digits_in_number_to_check: usize) -> bool {
    
    let mut seen_digits = [false; 10];

    get_digits!(number_to_check, number_of_digits_in_number_to_check)
        .all(|digit| {
            let already_seen = seen_digits[digit];
            seen_digits[digit] = true;
            !already_seen
        })
}

fn panic_on_digit_length_greater_than_base(k: usize) {
    if k > 10 {
        panic!("0 <= k <= 10. Passed in k = {}", k);
    }
}

fn get_smallest_permutation(k: usize) -> usize {
    panic_on_digit_length_greater_than_base(k);
    (0..k).fold(0, |acc, current| acc * 10 + current)
}

fn get_largest_permutation(k: usize) -> usize {
    panic_on_digit_length_greater_than_base(k);
    (0..k).fold(0, |acc, current| acc * 10 + 9 - current)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut operands: Vec<_> = input.split(" + ").collect();
    let last_two: Vec<_> = operands.last()?.split(" == ").take(2).collect();
    *operands.last_mut()? = last_two.get(0)?;
    let result = last_two.get(1)?;

    let mut constants: HashMap<char, isize> = HashMap::new();

    for operand in operands {
        for (i, letter) in operand.chars().rev().enumerate() {
            let exponent = u32::try_from(i).expect("usize exponent could not fit into u32.");
            *constants.entry(letter).or_default() += 10_isize.pow(exponent);
        }
    }

    for (i, letter) in result.chars().rev().enumerate() {
        let exponent = u32::try_from(i).expect("usize exponent could not fit into u32.");
        *constants.entry(letter).or_default() -= 10_isize.pow(exponent);
    }

    dbg!(&constants);
    dbg!(&result);

    let letters: Vec<_> = constants.keys().collect();
    let num_letters = letters.len();
    let smallest_permutation = get_smallest_permutation(num_letters);
    let largest_permutation = get_largest_permutation(num_letters);

    for permutation in smallest_permutation..=largest_permutation {
        if !has_unique_digits(permutation, num_letters) {
            continue;
        }

        let digits = get_digits!(permutation, num_letters);
    }
    None
}
