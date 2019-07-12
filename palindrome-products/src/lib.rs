pub type Palindrome = u64;
pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    fn is_palindrome(number: u64) -> bool {
        let number_of_digits = 1 + (number as f64).log10().floor() as u64;

        let get_digits_iterator = || (0..number_of_digits)
                                        .map(|digit_index| number / 10u64.pow(digit_index as u32) % 10);

        let number_of_digits_per_iterator = (number_of_digits / 2) as usize;

        let digits_forward = get_digits_iterator()
                                .take(number_of_digits_per_iterator);
        
        let digits_backward = get_digits_iterator()
                                .rev()
                                .take(number_of_digits_per_iterator);
        
        digits_forward.eq(digits_backward)
    }

    (min..=max)
        .flat_map(|i| (i..=max).map(move |j| i * j))
        .filter(|&product| is_palindrome(product))
        .collect()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().copied()
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().copied()
}
