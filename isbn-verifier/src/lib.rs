/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const X: u8 = 10;
    let mut cursor = 0;
    let mut sum = 0_usize;

    for character in isbn.chars() {
        let digit = if character.is_ascii_digit() {
            character as u8 - b'0'
        } else if character == 'X' {
            let is_check_digit = cursor == X - 1;

            if !is_check_digit {
                // X is somewhere before the check digit.
                return false;
            }

            X
        } else {
            continue;
        };

        if cursor == X {
            // Too many digits.
            return false;
        }

        sum += usize::from(digit * (X - cursor));
        cursor += 1;
    }

    cursor == X &&
        sum % 11 == 0
}
