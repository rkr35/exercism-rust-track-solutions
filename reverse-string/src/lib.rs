fn is_combining_diacritical_mark(c: char) -> bool {
    // http://www.fileformat.info/info/unicode/block/combining_diacritical_marks/index.htm
    const BLOCK_BEGIN: u16 = 0x300;
    const BLOCK_END: u16 = 0x36F;

    let mut buffer = [0; 2];
    let buffer = c.encode_utf16(&mut buffer);

    if buffer.len() != 1 {
        return false;
    }

    let code_point = buffer[0];
    code_point >= BLOCK_BEGIN && code_point <= BLOCK_END
}

pub fn reverse(input: &str) -> String {
    // First reverse the entire string without consideration for combining marks.
    let chars: Vec<_> = input.chars().rev().collect();

    // This vector holds the currently seen marks for the next non-marking character.
    let mut current_marks: Vec<char> = Vec::new();

    // This vector holds the characters of the final reversed string.
    // We'll be constructing a String from this vector.
    let mut current_reversed_string: Vec<char> = Vec::with_capacity(chars.len());

    for c in chars {
        if is_combining_diacritical_mark(c) {
            // We found another mark. Let's remember it.
            current_marks.push(c);
        } else {
            // Okay, so we found a non-marking character.

            // First we'll save this character.
            current_reversed_string.push(c);

            // Then we'll decorate this character by adding its marks.
            // Note: We reverse the order we add the marks because the marks were originally reversed
            // by the first call to .rev() above.
            current_reversed_string.extend(current_marks.iter().rev());

            // Then we clear our remembered marks for the next non-marking character.
            current_marks.clear();
        }
    }

    current_reversed_string.into_iter().collect()
}
