#![warn(clippy::all)]
#![warn(clippy::pedantic)]

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const X: usize = 10;

    isbn
        .chars()
        .filter_map(|character|
            if character.is_ascii_digit()                   { Some(character as usize - b'0' as usize) }
            else if character == 'X'                        { Some(X) } 
            else                                            { None }
        )
        .enumerate()
        .try_fold((0, 0), |(sum, _), (cursor, digit)| 
            if cursor >= X || digit == X && cursor != X - 1 { None }
            else                                            { Some((sum + digit * (X - cursor), cursor + 1)) }
        )
        .map_or(false, |(sum, cursor)| sum % (X + 1) == 0 && cursor == X)
}
