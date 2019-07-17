/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const MAX_ISBN_DIGIT_VALUE: u8 = 10;

    let mut digits_cursor = 0;
    let mut sum = 0_usize;

    for character in isbn.chars() {
        let digit = if character.is_ascii_digit() {
            character as u8 - b'0'
        } else if character == 'X' {
            let is_check_digit = digits_cursor == MAX_ISBN_DIGIT_VALUE - 1;

            if !is_check_digit {
                // X is somewhere before the check digit.
                return false;
            }

            MAX_ISBN_DIGIT_VALUE
        } else {
            continue;
        };

        if digits_cursor == MAX_ISBN_DIGIT_VALUE {
            // Too many digits.
            return false;
        }

        sum += usize::from(digit * (MAX_ISBN_DIGIT_VALUE - digits_cursor));
        digits_cursor += 1;
    }

    digits_cursor == MAX_ISBN_DIGIT_VALUE &&
        sum % 11 == 0
}
