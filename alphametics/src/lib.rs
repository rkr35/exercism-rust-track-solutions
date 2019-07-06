#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

fn has_unique_digits(number_to_check: usize,
                     number_of_digits_in_number_to_check: usize) -> bool {
    
    let mut seen_digits = [false; 10];

    for digit_index in 0..number_of_digits_in_number_to_check {
        let digit = number_to_check / 10_usize.pow(digit_index as u32);
        let digit = digit % 10;

        if seen_digits[digit] {
            return false;
        }

        seen_digits[digit] = true;
    }

    true
}

fn get_smallest_permutation(k: usize) -> usize {
    if k > 10 {
        panic!("0 <= k <= 10. Passed in k = {}", k);
    }

    // 1 -> 0
    // 2 -> 01
    // 3 -> 012
    // 4 -> 0123

    let mut permutation = 0;

    for i in 0..k {
        permutation *= 10;
        permutation += i;
    }

    permutation
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut operands: Vec<_> = input.split(" + ").collect();
    let last_two: Vec<_> = operands.last()?.split(" == ").take(2).collect();
    *operands.last_mut()? = last_two.get(0)?;
    let result = last_two.get(1)?;

    let mut letters: HashMap<char, isize> = HashMap::new();

    for operand in operands {
        for (i, letter) in operand.chars().rev().enumerate() {
            let exponent = u32::try_from(i).expect("usize exponent could not fit into u32.");
            *letters.entry(letter).or_default() += 10_isize.pow(exponent);
        }
    }

    for (i, letter) in result.chars().rev().enumerate() {
        let exponent = u32::try_from(i).expect("usize exponent could not fit into u32.");
        *letters.entry(letter).or_default() -= 10_isize.pow(exponent);
    }

    dbg!(&letters);
    dbg!(&result);

    let num_letters = letters.len();
    let num_permutations: usize = ((11 - num_letters)..=10).product();

    let smallest_permutation = get_smallest_permutation(num_letters);
    

    None
}
