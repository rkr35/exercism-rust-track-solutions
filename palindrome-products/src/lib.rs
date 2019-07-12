#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    use rayon::prelude::*;

    fn is_palindrome(number: u64) -> bool {
        let mut reversed = 0;
        let mut number_copy = number;

        // While there are digits to extract
        while number_copy > 0 {
            // Add the next digit to the reconstructed number.
            reversed = reversed * 10 + number_copy % 10;

            // Slide the number right one digit.
            number_copy /= 10;
        }

        reversed == number
    }

    (min..=max)
        .into_par_iter()
        .flat_map(|i| (i..=max).into_par_iter().map(move |j| i * j))
        .filter(|&product| is_palindrome(product))
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().copied()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().copied()
}
